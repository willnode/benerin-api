
# https://id.wiktionary.org/wiki/me-
def addPrefixMe(word: str):
    if len(word) == 0:
        return ('', '')
    elif word[0] in 'lmnrwy':
        return ('me' + word, 'v')
    elif word[0] in 'cdjt':
        if word[0] in 't':
            word = word[1:]
        return  ('men' + word, 'v')
    elif word[0] in 'bfpv':
        if word[0] in 'p':
            word = word[1:]
        return ('mem' + word, 'v')
    elif word[0] in 'aeghikou':
        if word[0] in 'k':
            word = word[1:]
        return ('meng' + word, 'v')
    elif word[0] in 'sz':
        return ('meny' + word[1:], 'v')
    else:
        return ('', '')

# https://id.wiktionary.org/wiki/ber-
def addPrefixBer(word: str):
    if len(word) == 0:
        return ('', '')
    elif word in {'ajar', 'unjur'}:
        return ('bel' + word, 'v')
    elif word[0] in 'r' or word[1:2] in {'er'}:
        return ('be' + word, 'v')
    else:
        return ('ber' + word, 'v')


# https://id.wiktionary.org/wiki/ter-
def addPrefixTer(word: str):
    if len(word) == 0:
        return ('', '')
    return ('ter' + word, 'a')

# https://id.wiktionary.org/wiki/di-
def addPrefixDi(word: str):
    if len(word) == 0:
        return ('', '')
    return ('di' + word, 'v')

# https://id.wiktionary.org/wiki/-an
def addSuffixAn(word: str):
    if len(word) == 0:
        return ('', '')
    return (word + 'an', 'n')
# https://id.wiktionary.org/wiki/-kan
def addSuffixKan(word: str):
    if len(word) == 0:
        return ('', '')
    return (word + 'kan', 'v')


# https://id.wiktionary.org/wiki/Wiktionary:Imbuhan_pe-_vs_peng-_vs_per-
def addPrefixPe(word: str):
    if len(word) == 0:
        return ('', '')
    return ('pe' + word, 'n')
def addPrefixPeng(word: str):
    if len(word) == 0:
        return ('', '')
    if word[0] == 'p':
        return ('pem' + word[1:], 'n')
    return ('peng' + word, 'n')
def addPrefixPer(word: str):
    if len(word) == 0 or word[0] == 'r':
        return ('', '')
    return ('per' + word, 'n')

# https://id.wiktionary.org/wiki/Templat:imbuhan_-ku
def addSuffixKu(word: str):
    if len(word) == 0:
        return ('', '')
    return (word + 'ku', 'n')
# https://id.wiktionary.org/wiki/Templat:imbuhan_-mu
def addSuffixMu(word: str):
    if len(word) == 0:
        return ('', '')
    return (word + 'mu', 'n')
# https://id.wiktionary.org/wiki/Templat:imbuhan_-nya
def addSuffixNya(word: str):
    if len(word) == 0:
        return ('', '')
    return (word + 'nya', 'n')

def doubleWord(word: str, pos: str):
    if len(word) == 0:
        return ('', '')
    if '-' in word:
        return ('', '')
    return (word + '-' + word, pos)
