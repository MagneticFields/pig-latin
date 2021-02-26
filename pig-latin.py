"""
This is an exercise from rust book. I tried to solve this with python first

Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

"""

def pig_latin(s: str) -> str:
    vovels = ['a', 'e', 'i', 'u', 'o']
    if s[0] in vovels:
        return s + '-h' +s[0] + 'y'
    else:
        return s[1:] + "-" + s[0] + "ay"


print(pig_latin("apple"))
print(pig_latin("fist"))


