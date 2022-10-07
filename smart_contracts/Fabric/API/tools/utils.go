// Author: Roland Kromes - R.G.Kromes@tudelft.nl

package tools

import (
	"encoding/hex"

	"fmt"
)

// Convinience function to decode multiple hex strings with fail-fast error
func decodeHexStrings(hexes ...string) (bytes [][]byte, err error) {
	bytes = make([][]byte, len(hexes))
	for i, h := range hexes {
		if bytes[i], err = hex.DecodeString(h); err != nil {
			return nil, fmt.Errorf("failed to decode hex string at position %d, %v", i, err)
		}
	}
	return
}
