# 🦀 Firmware Embarqué Asynchrone : Contrôle de Moteur Pas-à-Pas

Ce projet est un firmware complet développé en **Rust** pour un microcontrôleur **STM32**. Il utilise le framework asynchrone **Embassy** pour gérer de manière concurrente, réactive et sécurisée plusieurs périphériques matériels (encodeur, moteur, manette, LEDs).

Ce projet a été réalisé dans le cadre d'un TP sur les systèmes embarqués temps réel et met en pratique des concepts avancés de synchronisation logicielle et de gestion des interruptions.

## ⚙️ Matériel Requis (Hardware)

Le code est conçu pour fonctionner sur une carte STM32 équipée d'un Shield spécifique comprenant :
* **Un Encodeur Rotatif** en quadrature (QEI sur `TIM2`) avec un bouton poussoir intégré.
* **Un Moteur Pas-à-Pas** contrôlé via des impulsions PWM (`TIM3`) avec gestion de la direction et du microstepping.
* **Un Bargraph** composé de 8 LEDs (`GPIO Outputs`).
* **Un Gamepad** directionnel composé de 5 boutons (`GPIO Inputs`).

## 🚀 Fonctionnalités Principales

* **Contrôle dynamique du moteur :** La rotation de la molette (encodeur) ajuste en temps réel la vitesse et la direction du moteur pas-à-pas.
* **Retour visuel (Bargraph) :** Le niveau de vitesse est représenté visuellement sur les 8 LEDs du bargraph.
* **Arrêt d'urgence ultra-réactif :** Une pression sur la molette de l'encodeur déclenche une interruption matérielle (EXTI) qui coupe instantanément le moteur et bloque le système par sécurité.
* **Déverrouillage sécurisé :** Une fois en état d'urgence, le système refuse de redémarrer tant que l'utilisateur n'a pas appuyé sur le bouton central du Gamepad.

## 🧠 Architecture Logicielle (RTOS-like)

Pour garantir la réactivité du système et éviter que les opérations bloquantes ne ralentissent l'ensemble, le projet est découpé en **5 tâches asynchrones indépendantes** :

1. `encoder_task` *(Le Cerveau)* : Lit la position de l'encodeur toutes les 200ms, calcule la vitesse/direction, et publie ces ordres via des variables partagées.
2. `stepper_update_task` : Attend les signaux de commande en mode "sommeil" et ajuste les paramètres PWM du moteur.
3. `bargraph_task` : Attend les signaux de changement de niveau et met à jour l'affichage physique des LEDs.
4. `emergency_stop_task` : Tâche endormie, réveillée instantanément par une interruption matérielle (EXTI) sur front descendant. Elle coupe le moteur et lève un drapeau d'urgence (`AtomicBool`).
5. `gamepad_task` : Scrute l'état de la manette et permet de réinitialiser le drapeau d'urgence si le bouton central est pressé.

### 🔒 Synchronisation et Sécurité (Race Conditions)

L'architecture utilise des mécanismes de communication inter-tâches avancés :
* **Signaux (`embassy_sync::signal::Signal`) :** Agissent comme des "boîtes aux lettres" pour réveiller les tâches uniquement quand c'est nécessaire, libérant ainsi 100% du CPU.
* **Variables Atomiques (`AtomicU32`, `AtomicBool`) :** Permettent le partage d'états simples entre les tâches sans bloquer l'exécution.
* **Mutex (`embassy_sync::mutex::Mutex`) :** Utilisé pour protéger l'accès bas niveau au Timer 2 (`TIM2`). Il empêche une "race condition" critique où la tâche d'arrêt d'urgence tenterait de remettre le compteur à zéro exactement au même moment où la tâche encodeur tenterait de le lire.

## 📂 Structure du projet

```text
src/
├── main.rs       # Point d'entrée, initialisation du STM32, lancement des tâches et variables partagées
├── bargraph.rs   # Driver d'abstraction pour l'affichage LED (Générique via AnyPin)
├── encoder.rs    # Driver pour l'encodeur rotatif utilisant l'interface QEI et le PAC
├── stepper.rs    # Driver pour le moteur pas-à-pas utilisant un Timer PWM
└── gamepad.rs    # Driver pour la lecture du Gamepad
```

## 🛠️ Comment lancer le projet ?

### Prérequis
* Avoir [Rust et Cargo](https://www.rust-lang.org/tools/install) installés sur sa machine.

### Exécution
1. Clonez ce dépôt sur votre machine.
2. Ouvrez un terminal dans le dossier du projet.
3. Lancez la commande suivante :
   ```bash
   cargo run
