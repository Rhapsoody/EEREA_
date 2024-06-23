### ADR pour la Version 0.2 : Optimisation et Fiabilité

#### Contexte

Dans la version 0.1, nous avons établi les bases du projet avec la génération procédurale de la carte, la création de robots basiques, et l'exploration de la carte par ces derniers. 

Cependant, il paraissait intéressant de faire quelques optis. La version 0.2 vise à adresser ces limitations.

#### Décision

Nous avons décidé d'introduire des améliorations suivantes :

1. **Optimisation des Déplacements des Robots** :
    - **Algorithme de Recherche en Largeur (BFS)** : Pour optimiser les déplacements des robots vers leurs objectifs, nous avons implémenté l'algorithme BFS. Cette approche permet de trouver les chemins les plus courts dans un environnement avec des obstacles.
    - **Fonctionnalités Additionnelles** :
        - `move_towards_goal` : Dirige les robots vers des points spécifiques de manière efficace.

2. **Programmation Concurrentielle** :
    - **Tentative de Gestion Concurrentielle** : Nous avons initialement tenté d'introduire des threads pour gérer les déplacements des robots de manière concurrente. Cependant, en raison des complications et des risques de deadlocks, cette approche a été abandonnée.
    - **Annulation** : Nous avons rétabli une gestion basique des robots.

3. **Tests Unitaires** :
    - **Introduction de Tests Unitaires** : Pour assurer la fiabilité du code, nous avons introduit des tests unitaires pour les modules `robot`, `map`, `tile`, et `station`.
    - **Vérifications Clés** :
        - Initialisation correcte des tuiles et des stations.
        - Collecte correcte des données des robots par la station.
        - Création correcte des robots par la station et gestion de l'énergie.

#### Modules Impactés

1. **robot.rs** :
    - Ajout de la logique de déplacement optimisée.
    - Implémentation de nouvelles méthodes pour la gestion des comportements des robots.
    - Tests pour vérifier les comportements des robots.

2. **map.rs** :
    - Introduction de nouvelles méthodes de manipulation des tuiles.
    - Ajout de la génération procédurale de la carte avec une distribution réaliste des obstacles et des ressources.
    - Tests pour vérifier la génération correcte des cartes et la manipulation des tuiles.

3. **station.rs** :
    - Amélioration de la collecte et du partage des données entre les robots et la station.
    - Ajout de méthodes pour la gestion de l'énergie et la création de robots.
    - Tests pour vérifier la collecte des données et la gestion de l'énergie.

4. **tile.rs** :
    - Tests pour vérifier l'initialisation des tuiles et leur état.

5. **Tests** :
    - Création de fichiers de tests séparés pour les modules `robot`, `map`, `tile`, et `station`.
    - Mise en place de tests pour vérifier les fonctionnalités critiques des modules.

#### Conséquences

1. **Positives** :
    - **Optimisation** : Les déplacements des robots sont maintenant techniquement plus efficaces sauf pour la collecte ou c'etait un peu complexe.
    - **Fiabilité** : L'introduction des tests unitaires assure la robustesse du système et facilite la détection de régressions.

2. **Négatives** :
    - **Complexité Initiale** : La tentative d'introduction de la programmation concurrentielle a ajouté une complexité initiale, bien que nous ayons finalement décidé de revenir à une approche séquentielle.
