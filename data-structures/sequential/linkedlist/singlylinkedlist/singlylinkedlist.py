'''
EXPLANATION:
===========

'''


class ListNode:
    def __init__(self, data, next=None):
        self.data = data
        self.next = next


class SinglyLinkedList:
    def __init__(self):
        self.head = None
        self.size = 0

    def push(self, data):
        newListNode = ListNode(data, self.head)
        self.head = newListNode
        self.size += 1

    def pop(self):
        if (self.size != 0):
            self.head = self.head.next
            self.size -= 1

    def insert(self, data, index):
        if index > self.size or index < self.size:
            print("Error: Index out of bounds!")
        else:
            newNode = ListNode(data)
            start = self.head
            for x in range(0, index):
                start = start.next
            newNode.next = start
            start = newNode
            self.size += 1

    def remove(self, index):
        if index > self.size or index < self.size:
            print("Error: Index out of bounds!")
        else:
            start = self.head
            for x in range(0, index):
                start = start.next
            start = start.next
            self.size -= 1

    def printList(self):
        start = self.head
        arr = []
        while start.next != None:
            arr.append(start.data)
            start = start.next
        arr.append(start.data)
        print(arr)


def main():
    list = SinglyLinkedList()
    list.push(1)
    list.pop()
    list.push(2)
    list.push(3)
    list.push(4)
    list.printList()


if __name__ == '__main__':
    main()
