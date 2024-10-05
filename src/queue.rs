use std::collections::VecDeque;

pub struct Queue<T> {
    elements: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            elements: VecDeque::new(),
        }
    }

    // Adiciona um elemento no final da fila
    pub fn enqueue(&mut self, item: T) {
        self.elements.push_back(item);
    }

    // Remove um elemento do início da fila, retornando Option<T>
    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    // Verifica se a fila está vazia
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    // Retorna o tamanho da fila
    pub fn size(&self) -> usize {
        self.elements.len()
    }

    // Retorna o próximo elemento da fila sem removê-lo
    pub fn peek(&self) -> Option<&T> {
        self.elements.front()
    }
}

fn main() {
    let mut queue = Queue::new();
    
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    
    println!("Fila vazia? {}", queue.is_empty());
    println!("Tamanho da fila: {}", queue.size());
    
    println!("Próximo elemento: {:?}", queue.peek());

    println!("Removendo: {:?}", queue.dequeue());
    println!("Tamanho da fila: {}", queue.size());
}
