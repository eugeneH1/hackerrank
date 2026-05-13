function dayOfTheProgrammer(year: number): string {
  if (year < 1918) {
    return formateDate(year);
  }
  if (year === 1918) return "26.09.1918";
  const date = new Date(Date.UTC(year, 0, 1));
  date.setUTCDate(256);
  return formateDate(year, date.getMonth(), date.getDate());
}

function formateDate(year: number, month?: number, day?: number): string {
  if (!month) {
    const isJulianLeap = year % 4 === 0;
    return `${isJulianLeap ? 12 : 13}.09.${year}`;
  }
  const dd = day?.toString().padStart(2, "0");
  const mm = (month + 1).toString().padStart(2, "0");

  return `${dd}.${mm}.${year}`;
}
