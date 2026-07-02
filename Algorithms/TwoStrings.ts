function twoStrings(str1: string, str2: string): string {
    for(const char of str1) {
        if(str2.includes(char)) return "YES";
    }
    return "NO";
}
console.log(twoStrings("be", "cat"));