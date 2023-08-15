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

    def append(self):
        pass

    def remove(self):
        pass

    def insert(self, index):
        if (index > 0 and index < self.size - 1):
            pass

    def swapRemove(self, index):
        pass


def main():
    pass


if __name__ == '__main__':
    main()
