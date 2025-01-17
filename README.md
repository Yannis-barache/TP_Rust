# TP_Rust

## Membres du projet

[![Yannis BARACHE](https://img.shields.io/badge/Yannis%20Barache-000000?style=for-the-badge&logo=github&logoColor=white)](https://github.com/Yannis-barache)
[![Khalil ABADA](https://img.shields.io/badge/Khalil%20Abada-000000?style=for-the-badge&logo=github&logoColor=white)](https://github.com/Khalil-45)


## Description du projet

Ce projet est un TP noté en Rust pour le cours de Programmation avancée de BUT3 à l'IUT d'Orléans.

## Réponses aux questions

### Question 2

Une DynamicImage est une matrice de pixels convertible avec une représentation RGBA.

Pour convertir une DynamicImage en RGB8 il faut utiliser la méthode `to_rgb8()`.
```rust
let img = ImageReader::open("./img/iut.jpg")?.decode()?;
let img_rgb = img.to_rgb8();
```

### Question 3
L'image utilisé est `PNG_canal_alpha.png` qui est une image PNG avec un canal alpha.
![Image avec canal alpha](./img/PNG_canal_alpha.png)

En la convertissant en RGB8 on obtient une image sans canal alpha. Le canal alpha est donc supprimé et obtient une 
image avec des couleurs remplies.

![Image sans canal alpha](./img/Question3.png)

### Question 6

Pour obtenir la luminosité d'un pixel il faut utiliser la formule suivante :

`0.212671 * r + 0.715160 * g + 0.072169 * b`





### Question 9

Pour calculer la distance entre deux couleurs, selon la source [source pour la distance entre deux couleurs](https://fr.wikipedia.org/wiki/%C3%89cart_de_couleur), nommée delta E, elle est calculable ainsi :

![Formule écart](./img/calculEcart.svg)

Il est dit dans la source que l'on reconnait la formule de la distance euclidienne. J'ai trouvé un peu difficile la formule, j'ai donc utilisé la formule de la distance euclidienne avec le RGB. 

![Formule euclidienne RGB](./img/formule_euclidienne_rgb.png)

### Question 11
J'ai choisi la situation suivante : si la palette est vide, on ne change pas l'image et on la laisse telle qu'elle est, et nous prévenons l'utilisateur que la palette donnée est vide donc que ça n'a rien changé.

## Utilisation

## Notes
pallette vide

tramage aléatoire 
on dire un nombre aléatoire entre 0 et 1 et si le numéro du pixel est inférieur au seuil on le met en noir sinon en blanc
doit marcher aussi pour une palette de couleurs

### Trame de bayer
matrice qui détermine le seuil et qui permet le traitement de l'image. Quand la matrice est plus petite que l'image 
on la répète pour couvrir toute l'image. La matrice est défini par récurrence.

### Dithering
Technique de réduction de la profondeur de couleur d'une image. On utilise un tramage pour simuler des couleurs intermédiaires.

