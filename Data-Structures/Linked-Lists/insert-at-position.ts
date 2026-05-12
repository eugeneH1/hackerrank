function insertNodeAtPosition(llist, data, position) {
    let currPos = 0;
    let currNode = llist;
    
    while(currPos < position - 1) {
        currNode = currNode.next;
        currPos++;
    }
    let newNode = new SinglyLinkedListNode(data);
    // new node's next will point to what current node previously pointed to
    newNode.next = currNode.next;
    // current node's next needs to point to new node
    currNode.next = newNode;
    
    return llist;
}