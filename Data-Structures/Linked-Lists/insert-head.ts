function insertNodeAtHead(head, data) {
    const newHead = new SinglyLinkedListNode(data);
    newHead.next = head;
    return newHead; 
}