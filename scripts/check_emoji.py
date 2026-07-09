"""精确检查项目文件中的 emoji 字符（排除中文、box-drawing、数学符号、ASCII）"""
import os

# 精确 emoji 范围（排除中文、box-drawing、数学符号、箭头等 ASCII 范围）
EMOJI_RANGES = [
    (0x1F600, 0x1F64F),  # emoticons
    (0x1F300, 0x1F5FF),  # symbols & pictographs
    (0x1F680, 0x1F6FF),  # transport & map
    (0x1F700, 0x1F77F),  # alchemical symbols
    (0x1F780, 0x1F7FF),  # geometric shapes extended
    (0x1F800, 0x1F8FF),  # supplemental arrows-C
    (0x1F900, 0x1F9FF),  # supplemental symbols and pictographs
    (0x1FA00, 0x1FA6F),  # chess symbols
    (0x1FA70, 0x1FAFF),  # symbols and pictographs extended-A
    # Misc symbols - only specific emoji, not all
    # 0x2600-0x26FF contains many non-emoji symbols too
    # We check specific ones:
]

# 具体的 emoji 字符（Unicode 代码点）
SPECIFIC_EMOJI_CODEPOINTS = {
    0x2705,  # [OK] checkmark
    0x274C,  # [FAIL] cross mark
    0x26A0,  # [WARN] warning sign
    0x2139,  # [INFO] information
    0x2714,  # [OK] heavy check mark
    0x2716,  # [FAIL] heavy multiplication x
    0x2717,  # [FAIL] ballot x
    0x2718,  # [FAIL] heavy ballot x
    0x1F7E2, # [DONE] green circle
    0x1F7E1, # [IN_PROGRESS] yellow circle
    0x1F534, # [BLOCKED] red circle
    0x1F7E0, # [IN_PROGRESS] orange circle
    0x26AA,  # [TODO] white circle
    0x26AB,  # [TODO] black circle
    0x2B50,  # [STAR] star
    0x1F31F, # [STAR] glowing star
    0x1F4E6, # [PKG] package
    0x1F680, # [ROCKET] rocket
    0x1F527, # [TOOL] wrench
    0x2728,  # [STAR] sparkles
    0x1F41B, # [BUG] bug
    0x1F4DD, # [NOTE] memo
    0x1F3AF, # [TARGET] direct hit
    0x1F4A1, # [IDEA] bulb
    0x1F6A8, # [WARN] rotating light
    0x1F6AB, # [FAIL] no entry sign
    0x1F44D, # [OK] thumbs up
    0x1F44E, # [FAIL] thumbs down
    0x1F525, # [HOT] fire
    0x1F3C6, # [WIN] trophy
    0x1F4CC, # [PIN] pushpin
    0x1F511, # [KEY] key
    0x1F4A5, # [BOOM] collision
    0x1F4AF, # [100] hundred points
    0x1F916, # [BOT] robot
    0x1F4BB, # [PC] laptop
    0x1F5A5, # [PC] desktop computer
    0x1F4F1, # [PHONE] mobile phone
    0x1F4C1, # [DIR] file folder
    0x1F4C2, # [DIR] open file folder
    0x1F4C4, # [FILE] page facing up
    0x1F4C5, # [CAL] calendar
    0x1F4C8, # [CHART] chart increasing
    0x1F4C9, # [CHART] chart decreasing
    0x1F4CA, # [CHART] bar chart
    0x1F4CB, # [LIST] clipboard
    0x1F4CE, # [CLIP] paperclip
    0x1F4CF, # [RULER] straight ruler
    0x1F4D0, # [RULER] triangular ruler
    0x2702,  # [SCISSORS] scissors
    0x1F512, # [LOCK] locked
    0x1F513, # [UNLOCK] unlocked
    0x1F528, # [HAMMER] hammer
    0x1F529, # [NUT] nut and bolt
    0x2699,  # [GEAR] gear
    0x1F517, # [LINK] link
    0x1F9EA, # [TEST] test tube
    0x1F52C, # [MICRO] microscope
    0x1F52D, # [TELE] telescope
    0x1F525, # [HOT] fire
    0x1F6A7, # [CONST] construction
    0x1F6D1, # [STOP] stop sign
    0x1F4A3, # [BOMB] bomb
    0x1F4AC, # [CHAT] speech balloon
    0x1F4AD, # [THINK] thought balloon
    0x1F4A4, # [ZZZ] zzz
    0x1F4A0, # [DIAMOND] diamond shape
    0x1F48E, # [GEM] gem stone
    0x1F4FA, # [TV] television
    0x1F4FB, # [RADIO] radio
    0x1F4F7, # [CAMERA] camera
    0x1F4F8, # [CAMERA] camera with flash
    0x1F4F9, # [VIDEO] video camera
    0x1F3A5, # [FILM] movie camera
    0x1F4DE, # [PHONE] telephone receiver
    0x260E,  # [PHONE] black telephone
    0x1F4DF, # [PAGER] pager
    0x1F4E0, # [FAX] fax machine
    0x1F4E1, # [SATELLITE] satellite antenna
    0x1F50B, # [BATTERY] battery
    0x1F50C, # [PLUG] electric plug
    0x1F526, # [TORCH] flashlight
    0x1F56F, # [CANDLE] candle
    0x1F5D1, # [TRASH] wastebasket
    0x1F4B0, # [MONEY] money bag
    0x1F4B3, # [CARD] credit card
    0x1F4B9, # [CHART] chart with upwards trend
    0x1F4B2, # [DOLLAR] heavy dollar sign
    0x2709,  # [MAIL] envelope
    0x1F4E7, # [EMAIL] e-mail
    0x1F4E8, # [EMAIL] incoming envelope
    0x1F4E9, # [EMAIL] envelope with downwards arrow
    0x1F4EA, # [MAILBOX] closed mailbox
    0x1F4EB, # [MAILBOX] closed mailbox with raised flag
    0x1F4EC, # [MAILBOX] open mailbox with raised flag
    0x1F4ED, # [MAILBOX] open mailbox with lowered flag
    0x1F4EE, # [MAILBOX] postbox
    0x1F5F3, # [BALLOT] ballot box with ballot
    0x270F,  # [PENCIL] pencil
    0x2712,  # [PEN] black nib
    0x1F58B, # [PEN] fountain pen
    0x1F58A, # [PEN] pen
    0x1F4BC, # [BRIEFCASE] briefcase
    0x1F5C2, # [CARD] card index dividers
    0x1F5C3, # [BOX] card file box
    0x1F5C4, # [CABINET] file cabinet
    0x1F5FA, # [MAP] world map
    0x1F4CD, # [PIN] round pushpin
    0x1F5D2, # [NOTEPAD] spiral notepad
    0x1F5D3, # [CALENDAR] spiral calendar
    0x1F4C7, # [CARD] card index
    0x1F4C6, # [CALENDAR] tear-off calendar
    0x1F5DD, # [KEY] old key
    0x1F5E1, # [DAGGER] dagger knife
    0x2694,  # [SWORDS] crossed swords
    0x1F3F9, # [BOW] bow and arrow
    0x1F6E1, # [SHIELD] shield
    0x1F5DC, # [CLAMP] compression
    0x2696,  # [SCALES] scales
    0x1F9F2, # [MAGNET] magnet
    0x2697,  # [ALEMBIC] alembic
    0x1F9EC, # [DNA] dna
    0x1F321, # [THERMO] thermometer
    0x1F489, # [SYRINGE] syringe
    0x1F48A, # [PILL] pill
    0x1F9FA, # [TEST] basket
    0x1F9FB, # [SCROLL] roll of paper
    0x1F6BD, # [TOILET] toilet
    0x1F6B0, # [WATER] potable water
    0x1F6BF, # [SHOWER] shower
    0x1F6C1, # [BATH] bathtub
    0x1F9FC, # [SOAP] soap
    0x1F9F4, # [BOTTLE] lotion bottle
    0x1F9F7, # [PIN] safety pin
    0x1F6D2, # [CART] shopping cart
    0x1F6AA, # [DOOR] door
    0x1F6CF, # [BED] bed
    0x1F6CB, # [COUCH] couch and lamp
    0x1FA91, # [CHAIR] chair
    0x1F4A7, # [DROP] droplet
    0x1F4A8, # [DASH] dashing away
    0x1F4AB, # [DIZZY] dizzy
    0x1F4A6, # [SWEAT] sweat droplets
    0x1F4A2, # [ANGER] anger symbol
    0x1F4AE, # [WHITE_FLOWER] white flower
    0x1F4B1, # [CURRENCY] currency exchange
    0x1F4B4, # [YEN] yen banknote
    0x1F4B5, # [DOLLAR] dollar banknote
    0x1F4B6, # [EURO] euro banknote
    0x1F4B7, # [POUND] pound banknote
    0x1F4B8, # [MONEY] money with wings
    0x1F9FE, # [RECEIPT] receipt
    0x1F4F0, # [NEWSPAPER] newspaper
    0x1F4F2, # [PHONE] mobile phone with arrow
    0x1F4F3, # [VIBRATE] vibration mode
    0x1F4F4, # [SILENT] mobile phone off
    0x1F4F5, # [NO_PHONE] no mobile phones
    0x1F4F6, # [SIGNAL] antenna bars
    0x1F4FC, # [VHS] videocassette
    0x1F4FD, # [FILM] film projector
    0x1F500, # [SHUFFLE] shuffle tracks button
    0x1F501, # [REPEAT] repeat button
    0x1F502, # [REPEAT_ONE] repeat single button
    0x1F503, # [ARROWS] clockwise vertical arrows
    0x1F504, # [ARROWS] counterclockwise arrows button
    0x1F505, # [DIM] dim button
    0x1F506, # [BRIGHT] bright button
    0x1F507, # [MUTE] muted speaker
    0x1F508, # [SPEAKER] speaker low volume
    0x1F509, # [SPEAKER] speaker medium volume
    0x1F50A, # [LOUD] speaker high volume
    0x1F50D, # [SEARCH] magnifying glass tilted left
    0x1F50E, # [SEARCH] magnifying glass tilted right
    0x1F50F, # [LOCK] locked with pen
    0x1F510, # [LOCK] locked with key
    0x1F514, # [BELL] bell
    0x1F515, # [NO_BELL] bell with slash
    0x1F516, # [BOOKMARK] bookmark
    0x1F518, # [RADIO] radio button
    0x1F519, # [BACK] back arrow
    0x1F51A, # [END] end arrow
    0x1F51B, # [ON] on arrow
    0x1F51C, # [SOON] soon arrow
    0x1F51D, # [TOP] top arrow
    0x1F51E, # [NO_18] no one under eighteen
    0x1F51F, # [10] keycap 10
    0x1F520, # [ABCD] input latin uppercase
    0x1F521, # [abcd] input latin lowercase
    0x1F522, # [1234] input numbers
    0x1F523, # [SYMBOLS] input symbols
    0x1F524, # [ABC] input latin letters
    0x1F530, # [BEGINNER] japanese symbol for beginner
    0x1F531, # [TRIDENT] trident emblem
    0x1F532, # [BLACK_SQUARE] black square button
    0x1F533, # [WHITE_SQUARE] white square button
    0x1F535, # [TODO] blue circle
    0x1F536, # [DIAMOND] large orange diamond
    0x1F537, # [DIAMOND] large blue diamond
    0x1F538, # [DIAMOND] small orange diamond
    0x1F539, # [DIAMOND] small blue diamond
    0x1F53A, # [UP] red triangle pointed up
    0x1F53B, # [DOWN] red triangle pointed down
    0x1F53C, # [UP] up button
    0x1F53D, # [DOWN] down button
}

def is_emoji(char):
    cp = ord(char)
    # Skip ASCII range entirely
    if cp < 0x80:
        return False
    # Skip common non-emoji Unicode ranges
    # Chinese/Japanese/Korean
    if 0x4E00 <= cp <= 0x9FFF:
        return False
    if 0x3000 <= cp <= 0x303F:
        return False
    if 0xFF00 <= cp <= 0xFFEF:
        return False
    if 0x3040 <= cp <= 0x30FF:
        return False
    if 0xAC00 <= cp <= 0xD7AF:
        return False
    # Latin extended
    if 0x0080 <= cp <= 0x024F:
        return False
    # Combining diacritical marks
    if 0x0300 <= cp <= 0x036F:
        return False
    # Greek, Cyrillic
    if 0x0370 <= cp <= 0x04FF:
        return False
    # Box drawing characters
    if 0x2500 <= cp <= 0x257F:
        return False
    # Block elements
    if 0x2580 <= cp <= 0x259F:
        return False
    # Geometric shapes (most are not emoji)
    if 0x25A0 <= cp <= 0x25FF:
        return False
    # Arrows (most are not emoji)
    if 0x2190 <= cp <= 0x21FF:
        return False
    # Mathematical operators
    if 0x2200 <= cp <= 0x22FF:
        return False
    # Miscellaneous technical
    if 0x2300 <= cp <= 0x23FF:
        return False
    # Supplemental arrows
    if 0x27F0 <= cp <= 0x27FF:
        return False
    if 0x2900 <= cp <= 0x297F:
        return False
    # Variation selectors (standalone)
    if 0xFE00 <= cp <= 0xFE0F:
        return False
    # Check specific emoji codepoints
    if cp in SPECIFIC_EMOJI_CODEPOINTS:
        return True
    # Check emoji ranges
    for start, end in EMOJI_RANGES:
        if start <= cp <= end:
            return True
    return False

def has_emoji(line):
    return any(is_emoji(c) for c in line)

# 要检查的文件扩展名
EXTENSIONS = {'.md', '.rs', '.py', '.toml', '.yml', '.yaml', '.json', '.txt'}

# 要跳过的目录
SKIP_DIRS = {'.git', 'target', '.venv', '.venv-probe', '__pycache__', '.pytest_cache',
             'node_modules', 'ghidra_proj', 'iv8-ref'}

root = r'd:\dogepy\Tools\IV8'
found = []

for dirpath, dirnames, filenames in os.walk(root):
    dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]

    for filename in filenames:
        ext = os.path.splitext(filename)[1].lower()
        if ext not in EXTENSIONS:
            continue

        filepath = os.path.join(dirpath, filename)
        rel_path = os.path.relpath(filepath, root)

        try:
            with open(filepath, encoding='utf-8', errors='ignore') as f:
                for lineno, line in enumerate(f, 1):
                    if has_emoji(line):
                        emojis = [f"U+{ord(c):04X}" for c in line if is_emoji(c)]
                        found.append((rel_path, lineno, line.rstrip(), emojis))
        except Exception:
            pass

if not found:
    print("No emoji found in project files (excluding iv8-ref).")
else:
    print(f"Found {len(found)} lines with emoji:\n")
    for path, lineno, line, emojis in found:
        print(f"  {path}:{lineno}  emoji={emojis}")
        print(f"    {line[:100].encode('ascii', errors='replace').decode('ascii')}")
        print()
