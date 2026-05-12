function deleteNode(llist, position) {
    if(position === 0) return llist.next;
    let currPos = 0;
    let currNode = llist;
    
    while(currPos < position - 1) {
        currNode = currNode.next;
        currPos++;
    }
    // pointer at node before the one we want to delete
    currNode.next = currNode.next.next;
    return llist;
}