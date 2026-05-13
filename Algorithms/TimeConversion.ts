function timeConversion(s: string): string {
  let hour = Number(s.slice(0, 2));
  const modifier = s.slice(-2);
  const restOfTime = s.slice(2, -2);

  if (modifier === "PM") {
    if (hour !== 12) hour += 12;
  } else {
    if (hour === 12) hour = 0;
  }
  const hourString = hour.toString().padStart(2, '0');
  return `${hourString}${restOfTime}`;
}
