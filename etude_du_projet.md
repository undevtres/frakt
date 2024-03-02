# Participants au projet :

- **Lyès Slimani** : [https://github.com/eduardevs](https://github.com/eduardevs)
- **Eduardo Pina** : [https://github.com/eduardevs](https://github.com/eduardevs)
- **Fanny Guillemont** : [https://github.com/flammefolie](https://github.com/flammefolie)

## Séparation des tâches :

**Dans les grandes lignes :**

- Fanny s’est occupée de la génération des fractales.
- Lyès s’est occupé de la structure du code et des workers/serveurs.
- Eduardo s’est occupé des tests et des pipelines GitHub.

## Étude du travail :

### Partie networking :

Il a fallu dans un premier temps faire fonctionner la communication réseau et donc comprendre comment se font les divers
échanges.

- Dans un premier temps, on a essayé d’envoyer des requests et results faux pour être sûrs que la communication
  fonctionnait.
- Ensuite, pour tester le worker jusqu’à sa fin, nous avons fini la boucle de communication avec le serveur en envoyant
  des pixels non calculés (de couleur bleue).
- Pour séparer les responsabilités, un point d’entrée avait été laissé pour la partie fractale avec comme seules
  informations la tâche et la position du pixel demandé.

### Pour la partie Fractale :

Il a fallu tester comment cela fonctionnait en créant une image complète localement, sans quelconques notions de
networking ou de worker.

- Cela travaillait les notions de création de la crate `Complex` et de tous les traitements pour faire la translation
  entre une position du plan complexe et une position du tableau de pixels.

- Une fois le worker terminé, il a simplement fallu utiliser le point d’entrée avec la fractale et la position du pixel
  demandé et tout a fonctionné correctement.

### Du côté des tests et des pipelines :

- Nous avons d'abord établi les pipelines.
- Ensuite, nous avons écrit quelques tests.

## BONUS :

- Pipelines de vérifications avec un formatteur (`fmt`) et un linter (`clippy`).
