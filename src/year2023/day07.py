from functools import cmp_to_key

#data = """
#32T3K 765
#T55J5 684
#KK677 28
#KTJJT 220
#QQQJA 483    
#"""
with open("../../input/2023/real/day07.txt") as f:
    data = f.read()

data = data.split('\n')
data = [d.strip() for d in data if len(d.strip()) > 0]

five_of_a_kind = []
four_of_a_kind = []
full_house = []
three_of_a_kind = []
two_pair = []
one_pair = []
high_card = []

for line in data:
    cards, bid = line.split(' ')

    groups = {}
    for card in cards:
        if card in groups:
            groups[card] += 1
        else:
            groups[card] = 1

    if len(groups) == 1:
        five_of_a_kind.append(line)
    elif len(groups) == 2 and max([g for g in groups.values()]) == 4:
        four_of_a_kind.append(line)
    elif len(groups) == 2:
        full_house.append(line)
    elif len(groups) == 3 and max([g for g in groups.values()]) == 3:
        three_of_a_kind.append(line)
    elif len(groups) == 3:
        two_pair.append(line)
    elif len(groups) == 4:
        one_pair.append(line)
    else:
        high_card.append(line)
    
def comparecards(c1, c2):
    a, _ = c1.split(' ')
    b, _ = c2.split(' ')

    group_a = {}
    for card in a:
        if card in group_a:
            group_a[card] += 1
        else:
            group_a[card] = 1

    group_b = {}
    for card in b:
        if card in group_b:
            group_b[card] += 1
        else:
            group_b[card] = 1

    for group_length in [5, 4, 3, 2, 1]:
        lgroup_a = {k: v for (k, v) in group_a.items() if v == group_length}
        lgroup_b = {k: v for (k, v) in group_b.items() if v == group_length}

        if len(lgroup_a) == 0 and len(lgroup_b) == 0:
            continue

        for card in ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']:
            if card in lgroup_a and card in lgroup_b:
                continue;
            if card in lgroup_a and card not in lgroup_b:
                print(f"{lgroup_a} er større enn {lgroup_b}")
                return 1
            elif card not in lgroup_a and card in lgroup_b:
                print(f"{lgroup_a} er mindre enn {lgroup_b}")
                return -1

    print("Warning, equal decks")
    return 0; 

five_of_a_kind = sorted(five_of_a_kind, key=cmp_to_key(comparecards), reverse=True)
four_of_a_kind = sorted(four_of_a_kind, key=cmp_to_key(comparecards), reverse=True)
full_house = sorted(full_house, key=cmp_to_key(comparecards), reverse=True)
three_of_a_kind = sorted(three_of_a_kind, key=cmp_to_key(comparecards), reverse=True)
two_pair = sorted(two_pair, key=cmp_to_key(comparecards), reverse=True)
one_pair = sorted(one_pair, key=cmp_to_key(comparecards), reverse=True)
high_card = sorted(high_card, key=cmp_to_key(comparecards), reverse=True)

all_cards = [*five_of_a_kind, *four_of_a_kind, *full_house, *three_of_a_kind, *two_pair, *one_pair, *high_card]

score = 0
for (i, line) in enumerate(all_cards[::-1]):
    _, bid = line.split(' ')
    score += int(bid) * (i + 1)

print(score)

