# Sterling

The digital assistant that is one step ahead of you.

## Architecture

### High level Architecture

Sterling consist of a server that provides the main functionality, the server is composed by a shared core and many backend implementations over different LLM providers, and multiple front-ends that allow interacting from different platforms.

### Low level Architecture

The backend is composed of many crates that serve as modular components of the whole system.

### Sterling core

Provides traits that can be implemented by the front-ends, LLM backends, etc.

## Front-Ends

Front-ends are the main user interfaces to Sterling. They allow you to interact and connect with the different available functionality of the system. Currently, the available front-ends are:

### CLI

A Front-end that can be called from the command line.
