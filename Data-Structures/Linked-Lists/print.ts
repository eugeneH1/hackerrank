function printLinkedList(head) {
    let current = head;
    while(current !== null) {
        console.log(current.data);
        current = current.next;
    }
}