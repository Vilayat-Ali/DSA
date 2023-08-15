/*


 */

class ListNode {
    private int data;
    private ListNode next;

    public ListNode(int data, next: ListNode) {
        this.data = data;
        this.next = next;
    }
}

class SinglyLinkedList {
    private ListNode head;
    private int size;

    public SinglyLinkedList() {
        this.head = null;
        this.size = 0;
    }
}