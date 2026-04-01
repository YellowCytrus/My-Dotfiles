import requests
import json


BASE_URL = "https://edu.sfu-kras.ru/api/timetable"
GROUP_NAME = r"КИ23-14"

# Получаем список групп
r = requests.get(f"{BASE_URL}/groups", timeout=10)
r.raise_for_status()
groups = r.json()

# Находим нужную группу
group = None
for g in groups:
    if GROUP_NAME in g['name']:
        group = g
        break

if not group:
    raise ValueError(f"Группа {GROUP_NAME} не найдена")

print("Найдена группа:", group)

# Получаем расписание для группы
r = requests.get(f"{BASE_URL}/get&target={group['name']}", timeout=10)
r.raise_for_status()
schedule = r.json()


lessons = schedule['timetable']

weekdays = ['monday','tuesday','wednesday','thursday','friday','saturday','sunday']
schedule_json = {
    'odd': {day: [] for day in weekdays},
    'even': {day: [] for day in weekdays},
}

day_map = {'1':'monday','2':'tuesday','3':'wednesday','4':'thursday','5':'friday','6':'saturday','7':'sunday'}

for lesson in lessons:
    week_parity = 'odd' if lesson['week'] == '1' else 'even' if lesson['week'] == '2' else None
    if not week_parity:
        continue

    day = day_map.get(lesson['day'])
    if not day:
        continue

    time_to_pair = {
        '08:30-10:05': 1,
        '10:15-11:50': 2,
        '12:00-13:35': 3,
        '14:10-15:45': 4,
        '15:55-17:30': 5,
        '17:40-19:15': 6
    }
    pair_number = time_to_pair.get(lesson['time'], 0)

    schedule_json[week_parity][day].append({
        'pair': pair_number,
        'subject': lesson['subject'],
        'type': lesson['type'] if lesson['type'] else 'практика',
        'room': lesson['room'],
        'teacher': lesson['teacher']
    })

for parity in ['odd','even']:
    for day in weekdays:
        schedule_json[parity][day].sort(key=lambda x: x['pair'])

final_json = {
    "semester_start": "2024-01-08",
    "first_pair_time": "08:30",
    "pair_duration_minutes": 95,
    "break_duration_minutes": 10,
    "long_break_after_pair": 3,
    "long_break_duration_minutes": 35,
    "schedule": schedule_json
}

json_output = json.dumps(final_json, ensure_ascii=False, indent=2)

# print(json_output)

with open('schedule.json', 'w', encoding='utf-8') as f:
    f.write(json_output)