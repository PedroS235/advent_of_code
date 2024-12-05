import re

search = "XMAS"

word_search = []
while True:
    try:
        word_search.append(input())
    except EOFError:
        break

total_found = 0

horizontal = " ".join(word_search)
total_found += len(re.findall(search, horizontal)) + len(
    re.findall(search, horizontal[::-1])
)


word_search_rotated = []
for i in range(len(word_search[0]) - 1, -1, -1):
    t_row = ""
    for j in range(len(word_search)):
        t_row += word_search[j][i]
    word_search_rotated.append(t_row)

vertical = " ".join(word_search_rotated)
total_found += len(re.findall(search, vertical)) + len(
    re.findall(search, vertical[::-1])
)


total_found = 0


def get_diagonals(word_search):
    word_search_d = []
    for i in range(len(word_search)):
        x, y = 0, i
        row = ""
        while x < len(word_search[0]) and y < len(word_search):
            row += word_search[y][x]
            x += 1
            y += 1
        word_search_d.append(row)
    for i in range(1, len(word_search[0])):
        x, y = i, 0
        row = ""
        while x < len(word_search[0]) and y < len(word_search):
            row += word_search[y][x]
            x += 1
            y += 1
        word_search_d.append(row)
    return word_search_d


word_search_d = get_diagonals(word_search)
horizontal_d = " ".join(word_search_d)
total_found += len(re.findall(search, horizontal_d)) + len(
    re.findall(search, horizontal_d[::-1])
)

word_search_rotated_d = get_diagonals(word_search_rotated)
vertical_d = " ".join(word_search_rotated_d)
total_found += len(re.findall(search, vertical_d)) + len(
    re.findall(search, vertical_d[::-1])
)
print(total_found)
exit()


print(total_found)
