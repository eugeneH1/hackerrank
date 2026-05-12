function insertNodeAtTail(head, data) {
    if(!head) return new SinglyLinkedListNode(data);
    let currentNode = head;
    
    while(currentNode.next !== null) {
        currentNode = currentNode.next;
    }
    currentNode.next = new SinglyLinkedListNode(data);
    return head;
}