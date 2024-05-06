# Rust Log Collector

1. Log to file from client end from Kubernetes. 
2. Run collector. Read the files. Record the read file line number for fault tolerance.
3. Send the logs line by line to a store. This might be a local file for start and then might be AWS/Splunk/ES etc.
4. Configuration provision for collector. Tell the Pod and Container to Tail. Tell the Destination in plugin format. 
5. [OPTIONAL] Add Filter Mechanism.

1. Multithreaded
2. Fault Tolerant
3. External Connection
4. Plugin based architecture