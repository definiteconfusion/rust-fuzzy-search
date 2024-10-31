import subprocess
# Importable
def Fuzzy_Search(term:str="", terms:list[str]=[], max_dist:int = 3) -> list[str]:
    return subprocess.run(['./main', max_dist, term] + terms, capture_output=True).stdout.decode('utf-8').split('\n')[:-1]
# Local Use Only
fuzzySearch = lambda term, max_dist, terms: subprocess.run(['./main', max_dist, term] + terms, capture_output=True).stdout.decode('utf-8').split('\n')[:-1]