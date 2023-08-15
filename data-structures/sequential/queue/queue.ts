/*

EXPLANATION:
===========

*/

class ListNode<T> {
  private data: T;
  private next: ListNode<T> | null;

  constructor(data: T, next: ListNode<T> | null = null) {
    this.data = data;
    this.next = next;
  }

  // getters
  public getData(): T {
    return this.data;
  }
  public getNext(): ListNode<T> | null {
    return this.next;
  }

  // setters
  public setNext(nextListNode: ListNode<T>) {
    this.next = nextListNode;
  }
}

class SinglyLinkedList<T> {
  private head: ListNode<T> | null;
  private size: number;

  constructor() {
    this.head = null;
    this.size = 0;
  }

  public push(data: T) {
    const newListNode: ListNode<T> = new ListNode(data);
    if (this.head === null) {
      this.head = newListNode;
    } else {
      newListNode.setNext(this.head);
      this.head = newListNode;
    }
    this.size += 1;
  }

  public pop() {
    if (this.head !== null) {
      this.head = this.head.getNext() as ListNode<T>;
      this.size -= 1;
    }
  }

  public append() {}

  public remove(index: number) {}

  public swapRemove() {}

  // getters
  public getHead(): ListNode<T> | null {
    return this.head;
  }
  public getSize(): number {
    return this.size;
  }
}

// driver code
const main = () => {
  const linkedList: SinglyLinkedList<number> = new SinglyLinkedList();
  linkedList.push(1);
  linkedList.push(2);
  linkedList.pop();
  console.log(linkedList);
};

main();
