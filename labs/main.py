#!/usr/bin/env python3

import csv
import combiner
import os 

os.chdir(__file__[:__file__.rfind('/')])
words = []
replacements = {}

for addition in ['dt-adverbia.csv', 'dt-particles.csv', 'dt-pronouns.csv']:
    with open(addition, 'r', encoding='utf8') as f:
        reader = csv.reader(f, delimiter=',')
        for row in reader:
            replacements[','.join(row[0:2])] = row

with open('2-words.csv', 'r', encoding='utf8') as f:
    reader = csv.reader(f, delimiter=',')
    for row in reader:
        words.append((row[0], row[1]))

newwords = []

def addprefix(fun, *word):
    r = fun(*word)
    if r[0] != '':
        rkey = ','.join([r[0], r[1]])
        if rkey in replacements:
            rval = replacements.pop(rkey)
            newwords.append((r[0], word[0], r[1], rval[2]))
        else:  
            newwords.append((r[0], word[0], r[1], ''))
        return r
    else:
        return word
    
for word in words:
    addprefix(lambda *x: x, *word)
    if word[1] == 'v':
        dbl = addprefix(combiner.doubleWord, *word)

        addprefix(combiner.addPrefixBer, word[0])
        addprefix(combiner.addPrefixMe, word[0])
        addprefix(combiner.addPrefixTer, word[0])
        addprefix(combiner.addPrefixDi, word[0])
        addprefix(combiner.addPrefixPe, word[0])

        wordkan = addprefix(combiner.addSuffixKan, word[0])
        addprefix(combiner.addPrefixDi, wordkan[0])
        addprefix(combiner.addPrefixMe, wordkan[0])
        wordperkan = addprefix(combiner.addPrefixPer, wordkan[0])
        addprefix(combiner.addPrefixDi, wordperkan[0])
        addprefix(combiner.addPrefixMe, wordperkan[0])

        wordan = addprefix(combiner.addSuffixAn, word[0])
        addprefix(combiner.addSuffixKu, wordan[0])
        addprefix(combiner.addSuffixMu, wordan[0])
        addprefix(combiner.addSuffixNya, wordan[0])
    elif word[1] == 'n' or word[1] == 'num':
        dbl = addprefix(combiner.doubleWord, *word)

        addprefix(combiner.addSuffixKu, word[0])
        addprefix(combiner.addSuffixMu, word[0])
        addprefix(combiner.addSuffixNya, word[0])

        addprefix(combiner.addPrefixPe, word[0])
        addprefix(combiner.addPrefixPeng, word[0])
        addprefix(combiner.addPrefixPer, word[0])

        wordan = addprefix(combiner.addSuffixAn, word[0])
        addprefix(combiner.addPrefixPe, wordan[0])
        addprefix(combiner.addPrefixPeng, wordan[0])
        addprefix(combiner.addPrefixPer, wordan[0])

        addprefix(combiner.addPrefixBer, word[0])
        addprefix(combiner.addPrefixMe, wordan[0])
    elif word[1] == 'a':
        dbl = addprefix(combiner.doubleWord, *word)
        addprefix(combiner.addSuffixAn, dbl[0])

        wordkan = addprefix(combiner.addSuffixKan, word[0])
        addprefix(combiner.addPrefixDi, wordkan[0])
        addprefix(combiner.addSuffixKu, wordkan[0])
        addprefix(combiner.addSuffixMu, wordkan[0])
        addprefix(combiner.addSuffixNya, wordkan[0])

        wordme = addprefix(combiner.addPrefixMe, word[0])
        addprefix(combiner.addSuffixKu, wordme[0])
        addprefix(combiner.addSuffixMu, wordme[0])
        addprefix(combiner.addSuffixNya, wordme[0])


for rest in replacements.values():
    newwords.append((rest[0], '', rest[1], rest[2]))

newwords = [word for word in newwords if word[0] != '']

with open('3-combword.csv', 'w', encoding='utf8') as f:
    writer = csv.writer(f, delimiter=',')
    for word in newwords:
        writer.writerow(word)

