import csv 
import re

rows = set()
kinds = ['a', 'n', 'v', 'p', 'adv', 'num', 'pron']
with open('1-kbbi.csv', 'r', encoding='utf8') as f:
    reader = csv.reader(f)
    for row in reader:
        descs = row[2].split(';')
        word = re.search("^[\w -]+", row[0])
        if word is None:
            continue
        word = word.group(0).strip()
        if len(word) == 1 and word.upper() == word:
            continue
            
        for id, desc in enumerate(descs):
            descw = desc.strip().split(' ')
            if id != 0 and not descw[0].isdigit():
                continue
            kindidx = [i for i,x in enumerate(descw) if x in kinds]
            if len(kindidx) == 0:
                continue
            kind = descw[kindidx[0]]
            hint = ''
            # if kind in ['adv', 'p', 'pron']:
            #     try:
            #         kataidx = descw.index('kata')
            #         hint = descw[kataidx+1]
            #     except:
            #         pass

            rows.add((word, kind, hint))

srows = sorted(list(rows), key=lambda x: x[0])
with open('2-words.csv', 'w', encoding='utf8') as f:
    writer = csv.writer(f)
    writer.writerows(srows)


