type Unit = 'gram' | 'millilitre' | 'kilogram';

export interface Item = {
  id: string,
  name: string,
  unit: Unit,
  qty: number,
}

export interface ShoppingList = {
  id: string,
  title: string,
  date: Date,
  items: Item[],
}
