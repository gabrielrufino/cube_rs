#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn test_enqueue() {
        let mut queue = Queue::new();
        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);

        assert_eq!(queue.size(), 3);
        assert_eq!(queue.peek(), Some(&10));
    }

    #[test]
    fn test_dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);

        assert_eq!(queue.dequeue(), Some(10));
        assert_eq!(queue.dequeue(), Some(20));
        assert_eq!(queue.size(), 1);
        assert_eq!(queue.peek(), Some(&30));
    }

    #[test]
    fn test_is_empty() {
        let mut queue = Queue::new();

        assert!(queue.is_empty());

        queue.enqueue(1);
        assert!(!queue.is_empty());
    }

    #[test]
    fn test_size() {
        let mut queue = Queue::new();
        assert_eq!(queue.size(), 0);

        queue.enqueue(42);
        assert_eq!(queue.size(), 1);
        
        queue.enqueue(100);
        assert_eq!(queue.size(), 2);
        
        queue.dequeue();
        assert_eq!(queue.size(), 1);
    }

    #[test]
    fn test_peek() {
        let mut queue = Queue::new();
        assert_eq!(queue.peek(), None);

        queue.enqueue(15);
        assert_eq!(queue.peek(), Some(&15));

        queue.enqueue(25);
        assert_eq!(queue.peek(), Some(&15));

        queue.dequeue();
        assert_eq!(queue.peek(), Some(&25));
    }

    #[test]
    fn test_dequeue_empty() {
        let mut queue = Queue::new();
        assert_eq!(queue.dequeue(), None);
    }
}
