# ADR pour la Version 0.1

**Titre**: Projet EEREA - EFREI Paris - M2 Dev Manager FullStack

**Date**: 21/06/2024

## Contexte
Dans le cadre de nos coours au sein d'EFREI Paris, nous avons eu à mener un projet en Rust qui vise à simuler une collecte de ressources en essaim sur une carte 2D par des robots. 
La version 0.1 vise à établir les bases fonctionnelles du projet avec les fonctionnalités suivantes :
- Génération procédurale de la carte avec des obstacles et des ressources.
- Création de robots basiques avec des modules génériques.
- Exploration basique de la carte par les robots.
- Première implémentation d’une interface utilisateur avec `ggez` pour visualiser la carte et le mouvement des robots.

## Décision
1. **Technologie**:
   -  Le langage de programmation Rust pour sa sécurité de mémoire et ses performances.
   -  La bibliothèque `ggez` pour l'interface graphique afin de visualiser la carte et les robots.
   -  Le bruit de Perlin pour la génération procédurale des obstacles sur la carte.

2. **Carte**:
   - La carte est représentée en 2D avec des tuiles.
   - Les bords de la carte sont définis avec des obstacles.
   - Les obstacles et les ressources sont générés de manière procédurale en utilisant une fonction de bruit Perlin.
   - Trois types de ressources sont définis : énergie, minerais, lieux d'intérêt scientifique.

3. **Robots**:
   - Les robots sont modélisés de manière modulaire avec des modules spécialisés pour l'analyse, le forage et l'imagerie.
   - Les robots ont trois comportements principaux : exploration, collecte de ressources, et intérêt scientifique.
   - Les robots se déplacent de manière aléatoire sur la carte en évitant les obstacles.

4. **Station**:
   - La station centrale est initialisée sur une tuile vide (ni obstacle, ni ressource).
   - La station gère l'énergie et recharge les robots lorsqu'ils retournent à la station.
   - La station peut créer de nouveaux robots lorsque suffisamment d'énergie est disponible.

5. **Interface Utilisateur**:
   - Utilisation de `ggez` pour dessiner la carte, les obstacles, les ressources, les robots et la station.
   - Affichage des informations des robots avec une taille de police augmentée pour une meilleure lisibilité.

## Conséquences
- La structure du projet est maintenant en place, facilitant l'ajout de nouvelles fonctionnalités dans les versions futures.
- La génération procédurale de la carte permet une grande variabilité et rejouabilité.
- L'utilisation de `ggez` fournit une interface graphique simple et efficace, bien adaptée aux besoins du projet.
- La modularité des robots permettra d'ajouter facilement de nouveaux modules et comportements dans les versions futures.

## Notes
- Pour la version 0.2, nous prévoyons d'améliorer les modules des robots, d'implémenter un système de communication amélioré entre les robots et la station, et de gérer les conflits de données.

**Auteurs**: AGONGLO Shalom, MILLO Chelsey, SUTHARSAN Maanuja, WILLEKENS Elise