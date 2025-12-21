from functools import cmp_to_key

datatest = """
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483    
"""
with open("../../input/2023/real/day07.txt") as f:
    datareal = f.read()

data = datareal

data = data.split('\n')
data = [d.strip() for d in data if len(d.strip()) > 0]



def part1():
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

        for (l, r) in zip([c for c in a], [c for c in b]):
            if l == r:
                continue

            for card in ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']:
                if l == card:
                    return 1
                elif r == card:
                    return -1

        print("Decks are equal")
        return 0
        
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

    print(f"Part 1: {score}")


def part2():
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
        
        # Replace the J with the card you have most of
        if "J" in groups and len(groups) != 1:
            num_jokers = groups["J"]
            del groups["J"]
            max_card = max(groups, key=groups.get)
            groups[max_card] += num_jokers
    
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

        for (l, r) in zip([c for c in a], [c for c in b]):
            if l == r:
                continue

            for card in ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J']:
                if l == card:
                    return 1
                elif r == card:
                    return -1

        print("Decks are equal")
        return 0
        
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

    print(f"Part 2: {score}")


part1()
part2()

