function reverse(llist) {
    if(!llist || !llist.next) return llist;
    
    let newHead = reverse(llist.next);
    
    // Reaching this point means recursion is done and we're in call frame of node n - 1 (second last node) returning node n with it's null next property 
    llist.next.next = llist // node n's null next now points to node n - 1
    llist.next = null; // setting next property of current node to null, breaks circular reference
    
    // we return our new head of the linked list, it stays node n all the way back through the call stack
    return newHead;
}