# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [x] Commit: `Create Subscriber model struct.`
    -   [x] Commit: `Create Notification model struct.`
    -   [x] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [x] Commit: `Implement add function in Subscriber repository.`
    -   [x] Commit: `Implement list_all function in Subscriber repository.`
    -   [x] Commit: `Implement delete function in Subscriber repository.`
    -   [x] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [x] Commit: `Create Notification service struct skeleton.`
    -   [x] Commit: `Implement subscribe function in Notification service.`
    -   [x] Commit: `Implement subscribe function in Notification controller.`
    -   [x] Commit: `Implement unsubscribe function in Notification service.`
    -   [x] Commit: `Implement unsubscribe function in Notification controller.`
    -   [x] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [x] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [x] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [x] Commit: `Implement publish function in Program service and Program controller.`
    -   [x] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [x] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
1. In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. However, in this BambangShop case, a single Model struct is sufficient instead of using an interface (or trait in Rust). This is because we have a simple, uniform implementation for all subscribers - they all have the same attributes (url and name) and behavior. If we needed to support different types of subscribers with varying behaviors or attributes, then a trait would be beneficial to establish a common contract while allowing for different implementations. But since our subscribers all follow the same pattern and there's no polymorphic behavior needed, a struct is adequate and simpler.

2. Using DashMap (map/dictionary) is necessary rather than Vec (list) for this case because we need efficient lookups by unique identifiers. With a Vec, finding a specific subscriber would require iterating through the entire collection (O(n) time complexity), which becomes inefficient as the number of subscribers grows. DashMap provides O(1) lookup time complexity using the unique URL as the key, making operations like retrieving, updating, or deleting a specific subscriber much more efficient. Additionally, our repository is organized with a two-level map structure (product_type -> url -> Subscriber) which naturally fits a map data structure better than a list.

3. While we could implement the Singleton pattern instead of using DashMap, DashMap provides both thread safety and efficient concurrent access that would be challenging to implement manually. In Rust's ownership model, implementing a thread-safe singleton can be complex. DashMap handles concurrent read/write operations safely without blocking (lock-free), which is important for a system where multiple requests might access the subscriber list simultaneously. It combines the Singleton concept (we have a single global SUBSCRIBERS instance) with thread-safety guarantees. Implementing our own thread-safe singleton would likely require using mutex locks and more complex code, potentially with lower performance. DashMap provides a more elegant and efficient solution that aligns with Rust's emphasis on safe concurrency.

#### Reflection Publisher-2
1. Separating "Service" and "Repository" from the Model provides several key advantages based on design principles:
   - **Single Responsibility Principle**: Each component has a clearly defined responsibility - Models represent data structures, Repositories handle data access and storage, and Services contain business logic. This separation makes the code more maintainable and easier to understand.
   - **Separation of Concerns**: By isolating data access (Repository) from business rules (Service), we can change how data is stored without impacting business logic, and vice versa. This enables greater flexibility and adaptability.
   - **Testability**: Isolated components are easier to test. We can mock repositories when testing services, allowing us to test business logic without relying on actual data storage implementations.
   - **Scalability**: As the application grows, clear boundaries between components help manage complexity and allow different team members to work on different layers without conflicts.

2. If we only used Models without Services and Repositories, several problems would emerge:
   - **Code Complexity**: Each Model would need to contain data structure definitions, business logic, and data access code. For example, the Subscriber model would need to handle subscription rules, data validation, and storage operations.
   - **Tight Coupling**: Models would be directly dependent on each other. For instance, when a Product is created, it would directly interact with Notification and Subscriber models, creating a tightly coupled system where changes to one model could break others.
   - **Reduced Reusability**: Business logic embedded in models would be difficult to reuse across different parts of the application.
   - **Testing Difficulty**: Testing business logic would require setting up the entire data layer, making tests more complex and potentially brittle.

3. Postman has been valuable for testing this project in several ways:
   - **Request Organization**: The ability to organize related API calls into collections helps maintain a structured approach to testing different features (Products, Subscriptions, Notifications).
   - **Environment Variables**: Using variables for the base URL and other parameters makes it easy to switch between development, testing, and production environments.
   - **Request Chaining**: We can use data from one response in subsequent requests, which is particularly useful for testing the Observer pattern where we subscribe and then verify notifications.
   - **Automated Testing**: Postman's scripting capabilities allow writing test cases that can be run automatically, ensuring that the API behaves as expected after each code change.
   - **Request History**: Being able to see previous requests helps debug issues and understand the API's behavior over time.
   - **Collaboration**: Sharing collections with team members ensures everyone tests against the same endpoints with the same parameters, improving collaboration and reducing misunderstandings.

#### Reflection Publisher-3
1. In this tutorial case, we are using the **Push model** variation of the Observer Pattern. The publisher (BambangShop) actively pushes data to the subscribers whenever there is a relevant event (product creation, deletion, or promotion). This is evident in our implementation where:
   - The `NotificationService.notify()` method prepares notification payloads with all the necessary information about the product event
   - The `Subscriber.update()` method sends HTTP POST requests to subscribers with complete notification data
   - Subscribers don't need to request or pull any additional information; all relevant details are included in the notification payload

2. If we were to use the **Pull model** instead:

   **Advantages:**
   - **Reduced network traffic**: The publisher would only send minimal notifications that an event occurred, and subscribers would pull detailed information only when needed
   - **More subscriber control**: Subscribers could request only the specific information they need, when they need it
   - **Better handling of large data**: If product data is extensive, not sending it with every notification could save bandwidth

   **Disadvantages:**
   - **Increased latency**: Subscribers would need to make additional requests to get complete information, adding delay
   - **More complex implementation**: We would need to implement additional endpoints for subscribers to fetch product details
   - **Higher server load**: The publisher might face multiple simultaneous requests from subscribers after an event notification
   - **Potential data inconsistency**: By the time a subscriber pulls information, the product data might have changed again

3. If we did not use multi-threading in the notification process, several issues would arise:
   - **Blocking behavior**: Sending notifications would block the main thread, causing the API to become unresponsive during notification sending
   - **Poor user experience**: Users would experience significant delays when creating, deleting, or publishing products while the system waits for all notifications to complete
   - **Cascading failures**: If one subscriber is slow to respond or times out, it would delay the entire notification process and affect all other operations
   - **Limited scalability**: As the number of subscribers grows, the delay would increase linearly, making the system increasingly sluggish
   
   By using multi-threading (with `thread::spawn`), we ensure that notifications are sent asynchronously in the background, allowing the main API operations to respond quickly regardless of how many subscribers need to be notified or how long they take to receive notifications.
