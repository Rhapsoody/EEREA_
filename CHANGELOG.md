# Changelog

## Version 0.1

### Features
- Génération de la carte avec obstacles et ressources.
- Création de robots => modules génériques (analyse, forage, imagerie) + comportements comme forage.
- Déplacement des robots sur la carte de manière aléatoire.
- Création de la station.
- Gestion de l'énergie et de la collecte des ressources par les robots.
- Création de nouveaux robots lorsque la station dispose de suffisamment d'énergie.
- Utilisation de `ggez` pour afficher la carte, les robots et la station.
- Affichage des tuiles avec des images représentant les obstacles, les ressources et les tuiles vides + robots avec leurs infos.

### Fixes
- Répartition plus uniforme des obstacles en ajustant l'échelle du bruit de Perlin.
- Correction des bugs liés au déplacement des robots et à la gestion de l'énergie.

### Enhancements
- Augmentation de la taille de la police pour les informations des robots pour une meilleure lisibilité.


# Changelog

## Version 0.2

### Features
- Optimisation des déplacements des robots en utilisant l'algorithme de recherche en largeur (BFS).
- Ajout de la fonction `move_towards_goal` pour les déplacements des robots vers des objectifs spécifiques.
- Ajout de tests unitaires pour les modules `robot`, `map`, `tile`, et `station`.
- Vérification de l'initialisation correcte des tuiles, de la station, et de la gestion de l'énergie par les robots.
- Mise en place de fichiers de tests séparés pour chaque module, garantissant la robustesse du système.


### Enhancements
- Amélioration de la fiabilité du système en ajoutant des tests unitaires.
- Simplification de la gestion des comportements des robots pour une meilleure maintenabilité.
- Préparation du terrain pour des fonctionnalités plus complexes dans les futures versions.

