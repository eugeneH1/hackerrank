function reversePrint(llist) {
    if(!llist) return;
    if(llist.next) reversePrint(llist.next);
    console.log(llist.data);
}