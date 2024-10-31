import subprocess
# Importable
def Fuzzy_Search(term:str="", terms:list[str]=[]) -> list[str]:
    return subprocess.run(['./main', term] + terms, capture_output=True).stdout.decode('utf-8').split('\n')[:-1]
# Local Use Only
fuzzySearch = lambda term, terms: subprocess.run(['./main', term] + terms, capture_output=True).stdout.decode('utf-8').split('\n')[:-1]