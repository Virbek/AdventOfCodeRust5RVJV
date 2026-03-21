#include <iostream>
#include <string>
#include <fstream>
#include <stdlib.h>
#include <vector>
#include <chrono>

int main(){
    auto start = std::chrono::high_resolution_clock::now();

    int result = 0;

    int chest = 50;

    

    std::vector<std::string> combinaison;

    std::ifstream fichier("code.txt", std::ios::in);

    if(fichier){
        std::string contenu;
        while(getline(fichier, contenu)){
            combinaison.push_back(contenu);
        }

        fichier.close();
    }
    
    for(std::string c : combinaison){
        int taille = c.size();
        char direction = c[0];
        int tic = std::stoi(c.substr(1));
        for(int i = 0; i < tic; i++){
            if(direction == 'R'){
                chest++;
                if(chest > 99){
                    chest = 0;
                }
            }else{
                chest--;
                if(chest < 0){
                    chest = 99;
                }

            }
            if(chest == 0) result++;
        }
    }

    std::cout<<"Password is : "<< result<<'\n';

    auto stop = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::microseconds>(stop - start);

    std::cout << "Temps d'execution: " << duration.count() << " microsecondes" << std::endl;

    return 0;
}
