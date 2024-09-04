# Rust Log Collector

This is a starter Application to learn Rust. Each Iteration is planned as Medium Blogs with defined Milestones. We are going to build something like Fluentbit but completely in Rust. This application will have basic implementation for Rust code and Explain the code with comments and step-by-step with Medium Articles.

Happy Learning :D

### Roadmap for Event Collector

1. Log to file from client end like Kubernetes. 
2. Run collector. Read the files. Record the read file line number for fault tolerance.
3. Send the logs line by line to a store. This might be a local file for start and then might be AWS/Elast Search etc.
4. Configuration provision for collector. Tell the Pod and Container to Tail. Tell the Destination in plugin format. 
5. Add Filter Mechanism.

### Desired Features for the Application
1. Multi-threaded
2. Fault Tolerant
3. External Connection
4. Plugin based architecture

## Milestone 1 [Completed]
1. Dynamically Read Files from Folder
2. Record Counter and make sure to Read files from where we left off last.
3. Keep on Flushing events to Destination (another folder) on flush counter intervals.
4. All Metadata should be read from config file.

Blogs Covered
1. https://levelup.gitconnected.com/event-collector-your-first-rust-application-e0e7c2efa326 
2. https://medium.com/gitconnected/concurrency-in-rust-extending-your-application-using-tokio-8f6ed580e8aa 



## Milestone 2
1. Revamp the Application into cleaner code. Fix the Bugs.
2. Live Example of Files Running and Log Tailing.
3. Real Time metadata sharing between threads. [Blog]

    3.1:  We need to implement a thread lifecycle that tracks the state of threads in a shared data structure. 

    3.2: Every 5 seconds, each thread will update id, state(active) and lines processed.

    3.3: This is a metrics case. We wont use prometheus but record our own metrics.
4. Better design to handle multiple modes.
5. Containerize Everything.

## Milestone 3
1. Gather Performance Insights while running this in Kubernetes. [Blog]
2. Performance Optimizations.
3. Proper Parsing of Log Messages with Regex.
4. Regex based File Tailing.
5. Possibility of tailing multiple streams of files and separating them.
6. Implementing something like message pack.