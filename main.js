const levenshteinDistance = (a, b) => {
    const dp = Array(a.length + 1)
      .fill(null)
      .map(() => Array(b.length + 1).fill(0));
  
    for (let i = 0; i <= a.length; i++) dp[i][0] = i;
    for (let j = 0; j <= b.length; j++) dp[0][j] = j;
  
    for (let i = 1; i <= a.length; i++) {
      for (let j = 1; j <= b.length; j++) {
        if (a[i - 1] === b[j - 1]) {
          dp[i][j] = dp[i - 1][j - 1];
        } else {
          dp[i][j] = Math.min(dp[i - 1][j] + 1, dp[i][j - 1] + 1, dp[i - 1][j - 1] + 1);
        }
      }
    }
  
    return dp[a.length][b.length];
  };
  
  const fuzzySearch = (query, choices, maxDistance = 3) => {
    const results = choices
      .map(choice => {
        const distance = levenshteinDistance(query.toLowerCase(), choice.toLowerCase());
        return { choice, distance };
      })
      .filter(({ distance }) => distance <= maxDistance)
      .sort((a, b) => a.distance - b.distance);
  
    return results;
  };

export default fuzzySearch;