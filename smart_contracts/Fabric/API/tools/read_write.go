//  Copyright (c) 2022 TU Delft - IRIS project. All rights reserved.
// Author: Roland Kromes - R.G.Kromes@tudelft.nl

package tools

import (
	"fmt"
	"os"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

// // Read the data from the file called "fileName" with its extention ("fileExtentsion") at the path called "directory"
func ReadData(fileName string) ([]byte, int) {

	data, err := os.ReadFile(fileName)
	check(err)

	lengthOfData := len(data)

	return data, lengthOfData
}

// // Write the given "data" to the given "directory", the name of the file is given
// //by "fileName"dataChunks
func WriteData(data []byte, directory string, fileName string) {

	if _, err := os.Stat(directory); os.IsNotExist(err) {
		err := os.Mkdir(directory, 0777)
		check(err)
	}

	fileToWrite := directory + "/" + fileName

	f, err := os.Create(fileToWrite)
	check(err)

	defer f.Close()

	_, err1 := f.Write(data)
	check(err1)

}

func Hello() {
	fmt.Printf("Hello World ! \n")
}
