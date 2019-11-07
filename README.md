The goal of this application is to slowly create a complete **Natural Law / Resource Based Economy** resource management software for education and maybe practical use.

The present data model supports locations, unlimited number of parameters to describe every resource, multiple quantities for every resource and transformations of resources with arbitrary recursive dependency resolving.

However only a few of these functions are available at the moment and have to be done manually like the ability to record resources and their transformations into other resources.
Next step is to have fully automated order resolution based on resource dependencies and distance.

# Usage
After you add a resource, you can assign locations and amounts to it even in different units. When you want to for example turn flour into bread you make a transformation, then edit it and add all the events describing it.

An example for bread. First you add all the ingredients (resources). For example flour. Then you add and assign flour a parameter that states how it is moved around. Weight for example. Others can be moved by capacity, pieces etc. You also add a parameter that states how rough the flour is (text or number) and source plant (resource). Finally you assign a location to the flour and how much there is.

The baker who makes the bread (resource) wants to use smooth and wheat flour so he asks the system. The system shows him (after some calculation which is the second phase of the project) the most abundant flours with those two parameters and in the vicinity and he chooses one (in the form of a parameter) for his bread. He does the same for other ingredients. When he's ready he places an order (will be added later but it will create a "transformation" under the hood).

Now the system looks at his bread ingredients (resources) and calculates most efficient routes to his location and try to deliver them. It will decrease the amount of resources at the source locations (plus some energy for transport) and increase them at the destination. Once he has baked it, he completes the transformation with the last process which decreases his ingredients and increases the  number of bread at his location. This has to be done manually at the moment.

# Installation
You need a MySQL (or similar) database. The example data is found in sql/data.sql
To compile you need Rust nightly build with Cargo. I recommend https://rustup.rs