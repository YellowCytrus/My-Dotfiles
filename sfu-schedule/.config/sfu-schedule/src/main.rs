use chrono::{Datelike, Local, NaiveDate, NaiveTime, Timelike, Weekday};
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, CssProvider, Label, Orientation,
    ScrolledWindow,
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use serde::Deserialize;
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

const APP_ID: &str = "com.sfu.schedule";
const PANEL_WIDTH: i32 = 420;
const PANEL_MARGIN: i32 = 10;

#[derive(Debug, Deserialize, Clone)]
struct Lesson {
    pair: u32,
    subject: String,
    #[serde(rename = "type")]
    lesson_type: String,
    room: String,
    #[serde(default)]
    teacher: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct WeekSchedule {
    monday: Vec<Lesson>,
    tuesday: Vec<Lesson>,
    wednesday: Vec<Lesson>,
    thursday: Vec<Lesson>,
    friday: Vec<Lesson>,
    saturday: Vec<Lesson>,
    sunday: Vec<Lesson>,
}

#[derive(Debug, Deserialize, Clone)]
struct ScheduleData {
    odd: WeekSchedule,
    even: WeekSchedule,
}

#[derive(Debug, Deserialize, Clone)]
struct Schedule {
    semester_start: String,
    first_pair_time: String,
    pair_duration_minutes: u32,
    break_duration_minutes: u32,
    long_break_after_pair: u32,
    long_break_duration_minutes: u32,
    schedule: ScheduleData,
}

impl WeekSchedule {
    fn get_day(&self, weekday: Weekday) -> &Vec<Lesson> {
        match weekday {
            Weekday::Mon => &self.monday,
            Weekday::Tue => &self.tuesday,
            Weekday::Wed => &self.wednesday,
            Weekday::Thu => &self.thursday,
            Weekday::Fri => &self.friday,
            Weekday::Sat => &self.saturday,
            Weekday::Sun => &self.sunday,
        }
    }
}

fn get_config_path() -> PathBuf {
    if let Some(config_dir) = dirs::config_dir() {
        let app_config = config_dir.join("sfu-schedule");
        if app_config.join("schedule.json").exists() {
            return app_config.join("schedule.json");
        }
    }

    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()));

    if let Some(dir) = exe_dir {
        let path = dir.join("schedule.json");
        if path.exists() {
            return path;
        }
    }

    PathBuf::from("schedule.json")
}

fn load_schedule() -> Result<Schedule, String> {
    let path = get_config_path();
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Не удалось прочитать файл {:?}: {}", path, e))?;
    serde_json::from_str(&content).map_err(|e| format!("Ошибка парсинга JSON: {}", e))
}

fn is_odd_week(schedule: &Schedule) -> bool {
    let semester_start = NaiveDate::parse_from_str(&schedule.semester_start, "%Y-%m-%d")
        .expect("Неверный формат даты начала семестра");

    let today = Local::now().date_naive();
    let weeks_passed = (today - semester_start).num_weeks();

    // Первая неделя считается нечётной (odd)
    weeks_passed % 2 == 0
}

fn get_pair_time(schedule: &Schedule, pair_number: u32) -> (String, String) {
    let first_time = NaiveTime::parse_from_str(&schedule.first_pair_time, "%H:%M")
        .expect("Неверный формат времени первой пары");

    let mut start_minutes = first_time.hour() * 60 + first_time.minute();

    for i in 1..pair_number {
        start_minutes += schedule.pair_duration_minutes;
        if i == schedule.long_break_after_pair {
            start_minutes += schedule.long_break_duration_minutes;
        } else {
            start_minutes += schedule.break_duration_minutes;
        }
    }

    let end_minutes = start_minutes + schedule.pair_duration_minutes;

    let format_time = |mins: u32| format!("{:02}:{:02}", mins / 60, mins % 60);

    (format_time(start_minutes), format_time(end_minutes))
}

fn weekday_to_russian(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Mon => "Понедельник",
        Weekday::Tue => "Вторник",
        Weekday::Wed => "Среда",
        Weekday::Thu => "Четверг",
        Weekday::Fri => "Пятница",
        Weekday::Sat => "Суббота",
        Weekday::Sun => "Воскресенье",
    }
}

fn lesson_type_color(lesson_type: &str) -> &'static str {
    match lesson_type {
        "лекция" => "#7aa2f7",
        "практика" => "#98c379",
        "лаборатория" => "#ff9e64",
        _ => "#c0caf5",
    }
}

fn create_lesson_widget(schedule: &Schedule, lesson: &Lesson) -> GtkBox {
    let (start, end) = get_pair_time(schedule, lesson.pair);

    let lesson_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(4)
        .css_classes(["lesson-card"])
        .build();

    let header_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .build();

    let time_label = Label::builder()
        .label(&format!("{} – {}", start, end))
        .css_classes(["lesson-time"])
        .halign(Align::Start)
        .build();

    let pair_label = Label::builder()
        .label(&format!("{}п", lesson.pair))
        .css_classes(["lesson-pair"])
        .halign(Align::End)
        .hexpand(true)
        .build();

    header_box.append(&time_label);
    header_box.append(&pair_label);

    let subject_label = Label::builder()
        .label(&lesson.subject)
        .css_classes(["lesson-subject"])
        .halign(Align::Start)
        .wrap(true)
        .build();

    let type_color = lesson_type_color(&lesson.lesson_type);
    let info_text = if let Some(ref teacher) = lesson.teacher {
        format!(
            "<span color=\"{}\">{}</span> • {} • {}",
            type_color, lesson.lesson_type, lesson.room, teacher
        )
    } else {
        format!(
            "<span color=\"{}\">{}</span> • {}",
            type_color, lesson.lesson_type, lesson.room
        )
    };
    let info_label = Label::builder()
        .label(&info_text)
        .use_markup(true)
        .css_classes(["lesson-info"])
        .halign(Align::Start)
        .build();

    lesson_box.append(&header_box);
    lesson_box.append(&subject_label);
    lesson_box.append(&info_label);

    lesson_box
}

fn create_day_widget(
    schedule: &Schedule,
    day_name: &str,
    lessons: &[Lesson],
    is_today: bool,
) -> GtkBox {
    let day_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .css_classes(if is_today {
            vec!["day-card", "today"]
        } else {
            vec!["day-card"]
        })
        .build();

    let header = Label::builder()
        .label(day_name)
        .css_classes(["day-header"])
        .halign(Align::Start)
        .build();

    day_box.append(&header);

    if lessons.is_empty() {
        let no_lessons = Label::builder()
            .label("Нет пар")
            .css_classes(["no-lessons"])
            .halign(Align::Center)
            .build();
        day_box.append(&no_lessons);
    } else {
        let mut sorted_lessons = lessons.to_vec();
        sorted_lessons.sort_by_key(|l| l.pair);

        for lesson in &sorted_lessons {
            let lesson_widget = create_lesson_widget(schedule, lesson);
            day_box.append(&lesson_widget);
        }
    }

    day_box
}

fn create_today_section(schedule: &Schedule, week_schedule: &WeekSchedule) -> GtkBox {
    let today = Local::now().weekday();
    let lessons = week_schedule.get_day(today);
    let day_name = weekday_to_russian(today);

    let section = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .css_classes(["section"])
        .build();

    let title = Label::builder()
        .label("Сегодня")
        .css_classes(["section-title"])
        .halign(Align::Start)
        .build();

    section.append(&title);

    let day_widget = create_day_widget(schedule, day_name, lessons, true);
    section.append(&day_widget);

    section
}

fn create_week_section(schedule: &Schedule, week_schedule: &WeekSchedule, is_odd: bool) -> GtkBox {
    let today = Local::now().weekday();

    let section = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .css_classes(["section"])
        .build();

    let week_type = if is_odd { "нечётная" } else { "чётная" };
    let title = Label::builder()
        .label(&format!("Неделя ({})", week_type))
        .css_classes(["section-title"])
        .halign(Align::Start)
        .build();

    section.append(&title);

    let days_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .build();

    let weekdays = [
        (Weekday::Mon, "Понедельник"),
        (Weekday::Tue, "Вторник"),
        (Weekday::Wed, "Среда"),
        (Weekday::Thu, "Четверг"),
        (Weekday::Fri, "Пятница"),
        (Weekday::Sat, "Суббота"),
    ];

    for (weekday, name) in weekdays {
        let lessons = week_schedule.get_day(weekday);
        if !lessons.is_empty() || weekday == today {
            let is_today = weekday == today;
            let day_widget = create_day_widget(schedule, name, lessons, is_today);
            days_box.append(&day_widget);
        }
    }

    section.append(&days_box);

    section
}

fn build_content(schedule: &Schedule, show_odd: bool) -> GtkBox {
    let week_schedule = if show_odd {
        &schedule.schedule.odd
    } else {
        &schedule.schedule.even
    };

    let content_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .css_classes(["content-box"])
        .build();

    let today_section = create_today_section(schedule, week_schedule);
    content_box.append(&today_section);

    let week_section = create_week_section(schedule, week_schedule, show_odd);
    content_box.append(&week_section);

    content_box
}

static CSS_LOADED: AtomicBool = AtomicBool::new(false);

fn load_css() {
    if CSS_LOADED.load(Ordering::Acquire) {
        return;
    }

    let css = r#"
        * {
            font-family: "JetBrainsMono Nerd Font", monospace;
            font-size: 13px;
            color: #c0caf5;
            transition: all 0.2s ease;
        }

        /* Основное окно - в стиле swaync */
        window {
            background: rgba(26, 27, 38, 0.92);
            border: 1px solid #565f89;
            border-radius: 14px;
            box-shadow: 0 0 12px rgba(0, 0, 0, 0.4);
        }

        .main-container {
            padding: 8px;
            background: transparent;
        }

        /* Заголовок панели */
        .panel-header {
            padding: 0 0 8px 0;
            margin-bottom: 8px;
        }

        .header-row {
            margin-bottom: 6px;
        }

        .header {
            font-weight: 600;
            font-size: 14px;
            color: #c0caf5;
        }

        .week-indicator {
            font-size: 11px;
            color: #565f89;
        }

        /* Кнопка переключения недели - в стиле swaync */
        .week-toggle {
            background: #3e4451;
            color: #c0caf5;
            border: 1px solid #565f89;
            border-radius: 8px;
            padding: 6px 12px;
            margin: 0;
            font-size: 12px;
            font-weight: 600;
            min-height: 28px;
        }

        .week-toggle:hover {
            background: alpha(white, 0.08);
            border-color: #7aa2f7;
            color: #7aa2f7;
        }

        .week-toggle.active {
            background: alpha(#7aa2f7, 0.15);
            border-color: #7aa2f7;
            color: #7aa2f7;
        }

        /* Контент */
        .content-box {
            background: transparent;
        }

        /* Секции - в стиле notification */
        .section {
            background: #3e4451;
            border: 1px solid #565f89;
            border-radius: 12px;
            margin: 6px 0;
            padding: 10px;
        }

        .section:hover {
            background: alpha(white, 0.08);
            border-color: #7aa2f7;
        }

        .section-title {
            font-weight: 600;
            font-size: 14px;
            color: #c0caf5;
            margin-bottom: 4px;
        }

        /* Карточки дней - в стиле notification */
        .day-card {
            background: alpha(#3e4451, 0.6);
            border: 1px solid #565f89;
            border-radius: 10px;
            padding: 10px;
            margin-top: 6px;
        }

        .day-card.today {
            border-color: #7aa2f7;
            background: alpha(#7aa2f7, 0.15);
        }

        .day-header {
            font-weight: 600;
            font-size: 13px;
            color: #7dcfff;
            margin-bottom: 6px;
        }

        /* Карточки занятий - в стиле notification */
        .lesson-card {
            background: alpha(#3e4451, 0.8);
            border: 1px solid #565f89;
            border-radius: 8px;
            padding: 8px 10px;
            margin-top: 6px;
            border-left: 3px solid #7aa2f7;
        }

        .lesson-card:hover {
            background: alpha(white, 0.08);
            border-color: #7aa2f7;
        }

        .lesson-time {
            font-weight: 600;
            font-size: 12px;
            color: #73daca;
        }

        .lesson-pair {
            font-size: 11px;
            color: #565f89;
        }

        .lesson-subject {
            font-weight: 600;
            font-size: 13px;
            color: #c0caf5;
            margin-top: 4px;
        }

        .lesson-info {
            font-size: 11px;
            color: #abb2bf;
            margin-top: 4px;
        }

        .no-lessons {
            font-size: 12px;
            color: #565f89;
            font-style: italic;
            padding: 12px;
        }

        /* Скроллбар */
        scrolledwindow {
            background: transparent;
        }

        scrollbar {
            background: transparent;
            border: none;
        }

        scrollbar slider {
            background: rgba(86, 95, 137, 0.5);
            border-radius: 4px;
            min-width: 6px;
            min-height: 30px;
        }

        scrollbar slider:hover {
            background: rgba(122, 162, 247, 0.5);
        }
    "#;

    let provider = CssProvider::new();
    provider.load_from_data(css);

    if let Some(display) = gtk4::gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        CSS_LOADED.store(true, Ordering::Release);
    }
}

fn build_ui(app: &Application) {
    load_css();

    let schedule = match load_schedule() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Ошибка загрузки расписания: {}", e);
            let window = ApplicationWindow::builder()
                .application(app)
                .title("SFU Schedule - Ошибка")
                .default_width(300)
                .default_height(100)
                .build();

            let error_label = Label::new(Some(&format!("Ошибка: {}", e)));
            window.set_child(Some(&error_label));
            window.present();
            return;
        }
    };

    let current_is_odd = is_odd_week(&schedule);

    // Создаём окно
    let window = ApplicationWindow::builder()
        .application(app)
        .title("SFU Schedule")
        .default_width(PANEL_WIDTH)
        .default_height(600)
        .decorated(false)
        .resizable(false)
        .build();

    // Настраиваем layer-shell для позиционирования как панель
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::OnDemand);

    // Закрепляем справа сверху с отступами
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Right, true);
    window.set_anchor(Edge::Bottom, false);
    window.set_anchor(Edge::Left, false);

    window.set_margin(Edge::Top, PANEL_MARGIN + 40); // +40 для waybar
    window.set_margin(Edge::Right, PANEL_MARGIN);
    window.set_margin(Edge::Bottom, PANEL_MARGIN);

    // Состояние для переключения недели
    let show_odd = Rc::new(RefCell::new(current_is_odd));
    let schedule_rc = Rc::new(schedule);

    // Основной контейнер
    let main_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .css_classes(["main-container"])
        .build();

    // Заголовок панели
    let header_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .css_classes(["panel-header"])
        .build();

    let header_row = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .css_classes(["header-row"])
        .build();

    let header = Label::builder()
        .label("📚 Расписание")
        .css_classes(["header"])
        .halign(Align::Start)
        .hexpand(true)
        .build();

    // Кнопка переключения недели
    let week_type_text = if current_is_odd {
        "нечёт"
    } else {
        "чёт"
    };
    let toggle_btn = Button::builder()
        .label(&format!("⇄ {}", week_type_text))
        .css_classes(["week-toggle"])
        .build();

    header_row.append(&header);
    header_row.append(&toggle_btn);

    let today = Local::now();
    let week_type_full = if current_is_odd {
        "нечётная"
    } else {
        "чётная"
    };
    let week_indicator = Label::builder()
        .label(&format!(
            "{} • {} неделя",
            today.format("%d.%m.%Y"),
            week_type_full
        ))
        .css_classes(["week-indicator"])
        .halign(Align::Start)
        .build();

    header_box.append(&header_row);
    header_box.append(&week_indicator);

    main_box.append(&header_box);

    // Скроллируемый контент
    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .vexpand(true)
        .build();

    // Создаём начальный контент
    let content_box = Rc::new(RefCell::new(build_content(&schedule_rc, current_is_odd)));
    scrolled.set_child(Some(&*content_box.borrow()));

    main_box.append(&scrolled);

    // Обработчик переключения недели
    let content_box_clone = content_box.clone();
    let schedule_clone = schedule_rc.clone();
    let show_odd_clone = show_odd.clone();
    let scrolled_clone = scrolled.clone();

    toggle_btn.connect_clicked(move |btn| {
        let mut is_odd = show_odd_clone.borrow_mut();
        *is_odd = !*is_odd;

        // Обновляем текст кнопки
        let week_type_text = if *is_odd { "нечёт" } else { "чёт" };
        btn.set_label(&format!("⇄ {}", week_type_text));

        // Обновляем CSS класс
        if *is_odd {
            btn.remove_css_class("active");
        } else {
            btn.add_css_class("active");
        }

        // Пересоздаём контент
        let new_content = build_content(&schedule_clone, *is_odd);
        *content_box_clone.borrow_mut() = new_content;
        scrolled_clone.set_child(Some(&*content_box_clone.borrow()));
    });

    // Закрытие по Escape
    let window_clone = window.clone();
    let key_controller = gtk4::EventControllerKey::new();
    key_controller.connect_key_pressed(move |_, key, _, _| {
        if key == gtk4::gdk::Key::Escape {
            window_clone.close();
            glib::Propagation::Stop
        } else {
            glib::Propagation::Proceed
        }
    });
    window.add_controller(key_controller);

    window.set_child(Some(&main_box));
    window.present();
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}
