fn main() {
    println!("Hello, world!");
}

/*
#include <iostream>
#include <fstream>
#include <string>
#include <cstring>
#include <cmath>

std::string generatePassword(int length)
{
    char characters[] = {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '\'', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', '{', ']', '}', ';', ':', '"', '@', '#', '~', ',', '<', '.', '>', '/', '?'};

    int counter = 0;
    std::string password = "";

    while (counter < length)
    {
        double percentageProgress = ((double)counter / (double)length * 100.0) + 1.0;
        if (std::fmod(percentageProgress, 1) == 0)
        {
            std::cout << percentageProgress << " complete." << std::endl;
        }
        // password.append(characters[Math.round(random.nextInt(characters.length))]); // in case it doesn't work
        password += characters[rand() % std::strlen(characters)];
        counter++;
    }

    return password;
}

int main(int argc, char const *argv[])
{
    std::cout << "Enter number of characters: ";
    int numberOfCharacters;
    std::cin >> numberOfCharacters;

    std::cout << "Generating random password..." << std::endl;
    std::string password = generatePassword(numberOfCharacters);

    std::cout << "Saving password to file..." << std::endl;
    std::ofstream out("password.txt");

    if (out)
    {
        out << password;
        out.close();
        std::cout << "Password has been saved to file." << std::endl;
        return 0;
    }
    else
    {
        std::cout << "An error occurred." << std::endl;
        return 1;
    }
}
*/