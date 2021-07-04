import { Interval } from './interval';

export const DomDriverJsInterval = Interval;

export class MyClass {
    private number_inner: number;

    constructor() {
        console.info("To jest konstruktor MyClass444");
        this.number_inner = 42;
    }

    get number() {
        return this.number_inner;
    }

    set number(n: number) {
        this.number_inner = n;
    }

    render() {
        return `My number is: ${this.number}`;
    }
}

