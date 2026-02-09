#include <iostream>
#include <string>
#include <fstream>
#include <vector>
#include <chrono>

int main() {
    auto start = std::chrono::high_resolution_clock::now();

    // Lecture du fichier
    std::vector<std::string> combinaison;
    std::ifstream fichier("code.txt", std::ios::in);

    if (fichier) {
        std::string contenu;
        // Le fichier sépare les plages par des virgules
        while (getline(fichier, contenu, ',')) {
            combinaison.push_back(contenu);
        }
        fichier.close();
    } else {
        std::cout << "Erreur : Impossible d'ouvrir le fichier code.txt" << std::endl;
        return 1;
    }

    // On utilise long long pour la somme finale car elle sera gigantesque
    long long sommeTotale = 0;

    for (std::string c : combinaison) {
        // Nettoyage éventuel des sauts de ligne ou espaces résiduels
        if (c.empty()) continue;

        int sep = c.find('-');
        if (sep == std::string::npos) continue; // Sécurité si format bizarre

        // CONVERSION EN LONG LONG (CRUCIAL POUR EVITER LE BUG NEGATIF)
        long long scode = std::stoll(c.substr(0, sep));
        long long ecode = std::stoll(c.substr(sep + 1));

        // ALGORITHME GENERATIF
        // On génère les nombres "moitiés" : 1, 2... 10... 100...
        // Pour ecode = 2121212124 (10 chiffres), la moitié a 5 chiffres.
        // Aller jusqu'à 200 000 suffit largement à couvrir tous les cas possibles.
        for (long long i = 1; i < 200000; i++) {
            
            // 1. On fabrique le nombre répété (ex: i=12 -> "1212")
            std::string sHalf = std::to_string(i);
            std::string sFull = sHalf + sHalf;

            // 2. On convertit en nombre
            long long candidat = std::stoll(sFull);

            // 3. OPTIMISATION : Si le candidat dépasse la fin de la plage actuelle,
            // inutile de tester les suivants (qui seront encore plus grands).
            // On passe à la plage suivante du fichier.
            if (candidat > ecode) {
                break;
            }

            // 4. VERIFICATION : Si le candidat est au-dessus du début de la plage
            if (candidat >= scode) {
                // L'énoncé dit : "Adding up all the invalid IDs"
                // Donc on ajoute le candidat (ex: 1212), pas la moitié.
                sommeTotale += candidat;
                
                // Debug optionnel pour vérifier
                // std::cout << "Trouve: " << candidat << " dans la plage " << c << "\n";
            }
        }
    }

    std::cout << "Reponse finale (Somme des ID invalides) : " << sommeTotale << '\n';

    auto stop = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::microseconds>(stop - start);
    std::cout << "Temps d'execution : " << duration.count() << " microsecondes" << std::endl;

    return 0;
}