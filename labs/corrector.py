import csv
from tqdm import tqdm

# read all from combword
words = set()
with open('3-combword.csv', 'r', encoding='utf8') as f:
    reader = csv.reader(f, delimiter=',')
    for line in reader:
        if ' ' not in line[0]:
            words.add(line[0])

corrections = {}

def edits(word):
    "All edits that are one edit away from `word`."
    letters    = 'abcdefghijklmnopqrstuvwxyz'
    splits     = [(word[:i], word[i:])    for i in range(len(word) + 1)]
    deletes    = [L + R[1:]               for L, R in splits if R]
    transposes = [L + R[1] + R[0] + R[2:] for L, R in splits if len(R)>1]
    replaces   = [L + c + R[1:]           for L, R in splits if R for c in letters]
    inserts    = [L + c + R               for L, R in splits for c in letters]
    return set(deletes + transposes + replaces + inserts)

for w in tqdm(words):
    for e1 in edits(w):
        if e1 not in words:
            corrections[e1] = w
        for e2 in edits(e1):
            if e2 not in words:
                corrections[e2] = w

with open('4-corrections.csv', 'w', encoding='utf8') as f:
    writer = csv.writer(f, delimiter=',')
    for k, v in corrections.items():
        writer.writerow([k, v])