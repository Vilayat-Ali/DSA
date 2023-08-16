/*


 */

import java.lang.*;
import java.util.*;

class ListNode<T> {
    private T data;
    private Optional<ListNode> next;

    public ListNode(T data) {
        this.data = data;
        this.next = null;
    }

    public T getData() {
        return this.data;
    }

    public void setData(T data) {
        this.data = data;
    }

    public Optional<ListNode> getNext() {
        return this.next;
    }

    public void setNext(Optional<ListNode> next) {
        this.next = next;
    }
}

class SinglyLinkedList<T> {
    private ListNode<T> head;
    private int size;

    public SinglyLinkedList() {
        this.head = null;
        this.size = 0;
    }

    public void push(T data) {
        ListNode<T> newNode = new ListNode<T>(data);
        newNode.setNext(Optional.of(this.head));
        this.head = newNode;
        this.size += 1;
    }

    public int getSize() {
        return this.size;
    }

}

class Driver {
    public static void main(String[] args) {
        Scanner scannerOb = new Scanner(System.in);
        int size;
        System.out.println("Enter size: ");
        size = scannerOb.nextInt();

        SinglyLinkedList<Integer> list = new SinglyLinkedList<Integer>();

        for(int i=0; i<size; i++){
            System.out.println("Enter a number: ");
            int input = scannerOb.nextInt();
            list.push(input);
        }

        System.out.println("Size: "+list.getSize());
    }
}