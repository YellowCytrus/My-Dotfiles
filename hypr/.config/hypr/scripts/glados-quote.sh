#!/bin/bash
# Random GLaDOS quote — displayed after fastfetch

DIM='\033[38;2;122;122;144m'
WHITE='\033[38;2;208;208;224m'
RESET='\033[0m'

quotes=(
    "The Enrichment Center reminds you that the Weighted Companion Cube cannot speak."
    "This was a triumph. I'm making a note here: huge success."
    "We do what we must because we can."
    "The cake is a lie."
    "For the good of all of us, except the ones who are dead."
    "Please note that we have added a conditions supplement to the testing track."
    "Did you know you can donate one or all of your vital organs to the Aperture Science Self-Esteem Fund for Girls?"
    "All Aperture technologies remain safely operational up to 4000 degrees Kelvin."
    "The Enrichment Center is committed to the well-being of all participants."
    "Momentum, a function of mass and velocity, is conserved between portals."
    "Unbelievable. You, [subject name here], must be the pride of [subject hometown here]."
    "Well done. Here come the test results: You are a horrible person."
    "Science isn't about WHY. It's about WHY NOT."
    "When life gives you lemons, don't make lemonade. Make life take the lemons back!"
    "I'm not even angry. I'm being so sincere right now."
    "There really was a cake."
    "The Enrichment Center promises to always provide a safe testing environment."
    "Thank you for assuming the party escort submission position."
    "Aperture Science: We do what we must, because we can."
    "Remember, the Aperture Science Bring Your Daughter to Work Day is the perfect time to have her tested."
)

idx=$(( RANDOM % ${#quotes[@]} ))

echo -e " ${DIM}\"${WHITE}${quotes[$idx]}${DIM}\"  — GLaDOS${RESET}"
echo ""
