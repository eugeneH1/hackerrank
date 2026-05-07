function acmTeam(topic: string[]): number[] {
  let max = 0;
  let maxTeams = 0;
  for (let i = 0; i < topic.length - 1; i++) {
    for (let j = i; j < topic.length; j++) {
      const a = parseInt(topic[i], 2);
      const b = parseInt(topic[j], 2);
      const topics = ((a | b).toString(2).match(/1/g) || []).length;
      if (topics > max) {
        maxTeams = 1;
        max = topics;
      } else if (topics == max) {
        maxTeams++;
      }
      console.log("topics", topics);
    }
  }
  return [max, maxTeams];
}

const topic: string[] = ['10101', '11100', '11010', '00101'];

const topics2: string[] = [
  '11101', // 29
  '10101', // 21
  '11001', // 25
  '10111', // 23
  '10000', // 16
  '01110'  // 14
];
console.log(acmTeam(topics2))
