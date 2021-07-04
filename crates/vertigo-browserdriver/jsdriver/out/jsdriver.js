class Interval {
    constructor(callback) {
        this.callback = callback;
    }
    set_interval(duration, callback_id) {
        const timer_id = setInterval(() => {
            this.callback(callback_id);
        }, duration);
        return timer_id;
    }
    clear_interval(timer_id) {
        clearInterval(timer_id);
    }
}

const DomDriverJsInterval = Interval;
class MyClass {
    constructor() {
        console.info("To jest konstruktor MyClass444");
        this.number_inner = 42;
    }
    get number() {
        return this.number_inner;
    }
    set number(n) {
        this.number_inner = n;
    }
    render() {
        return `My number is: ${this.number}`;
    }
}

export { DomDriverJsInterval, MyClass };
