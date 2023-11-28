#include <stdio.h>   // Inclut la bibliothèque standard d'entrée/sortie.
#include <pthread.h> // Inclut la bibliothèque pthread pour la programmation multi-thread.

#define NUM_THREADS 10   // Définit le nombre de threads à créer.
#define NUM_INCREMENTS 1000 // Définit le nombre d'incrémentations par thread.

int shared_variable = 0; // Déclare une variable partagée entre tous les threads.

// Fonction exécutée par chaque thread.
void *increment(void *arg) {
    for (int i = 0; i < NUM_INCREMENTS; i++) {
        shared_variable++; // Incrémente la variable partagée.
    }
    return NULL; // Termine la fonction du thread.
}

int main() {
    pthread_t threads[NUM_THREADS]; // Tableau pour stocker les identifiants de threads.

    // Création des threads.
    for (int i = 0; i < NUM_THREADS; i++) {
        // Crée un nouveau thread qui exécutera la fonction 'increment'.
        if (pthread_create(&threads[i], NULL, increment, NULL) != 0) {
            perror("Failed to create thread"); // Affiche un message d'erreur si la création échoue.
            return 1; // Termine le programme avec un code d'erreur.
        }
    }

    // Boucle pour attendre la fin de tous les threads.
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_join(threads[i], NULL); // Attend que le thread 'i' se termine.
    }

    // Affiche le résultat final de la variable partagée.
    printf("Résultat final : %d\n", shared_variable);
    return 0; // Termine le programme avec succès.
}