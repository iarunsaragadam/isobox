package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"time"
)

type ExecuteRequest struct {
	Language string `json:"language"`
	Code     string `json:"code"`
}

type ExecuteResponse struct {
	Stdout   string `json:"stdout"`
	Stderr   string `json:"stderr"`
	ExitCode int    `json:"exit_code"`
}

func executeCode(client *http.Client, language, code string) (*ExecuteResponse, error) {
	request := ExecuteRequest{
		Language: language,
		Code:     code,
	}

	jsonData, err := json.Marshal(request)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal request: %v", err)
	}

	resp, err := client.Post("http://localhost:8000/execute", "application/json", bytes.NewBuffer(jsonData))
	if err != nil {
		return nil, fmt.Errorf("failed to make request: %v", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response body: %v", err)
	}

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("API returned status %d: %s", resp.StatusCode, string(body))
	}

	var response ExecuteResponse
	if err := json.Unmarshal(body, &response); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %v", err)
	}

	return &response, nil
}

func main() {
	client := &http.Client{
		Timeout: 30 * time.Second,
	}

	fmt.Println("🚀 Testing isobox API with Go code examples...\n")

	// Test 1: Basic Go program
	fmt.Println("=== Test 1: Basic Go Program ===")
	basicCode := `package main

import "fmt"

func main() {
	fmt.Println("Hello from Go!")
	fmt.Println("Testing isobox API...")
}`

	result1, err := executeCode(client, "go", basicCode)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	} else {
		fmt.Printf("Exit Code: %d\n", result1.ExitCode)
		fmt.Printf("Stdout: %s", result1.Stdout)
		if result1.Stderr != "" {
			fmt.Printf("Stderr: %s", result1.Stderr)
		}
	}
	fmt.Println()

	// Test 2: Math operations
	fmt.Println("=== Test 2: Math Operations ===")
	mathCode := `package main

import (
	"fmt"
	"math"
)

func main() {
	fmt.Println("Math operations:")
	fmt.Printf("π = %.6f\n", math.Pi)
	fmt.Printf("e = %.6f\n", math.E)
	fmt.Printf("√16 = %.2f\n", math.Sqrt(16))
	fmt.Printf("2^8 = %.0f\n", math.Pow(2, 8))
}`

	result2, err := executeCode(client, "go", mathCode)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	} else {
		fmt.Printf("Exit Code: %d\n", result2.ExitCode)
		fmt.Printf("Stdout: %s", result2.Stdout)
		if result2.Stderr != "" {
			fmt.Printf("Stderr: %s", result2.Stderr)
		}
	}
	fmt.Println()

	// Test 3: String manipulation
	fmt.Println("=== Test 3: String Manipulation ===")
	stringCode := `package main

import (
	"fmt"
	"strings"
)

func main() {
	text := "Hello, isobox! Welcome to secure code execution."
	fmt.Println("Original:", text)
	fmt.Println("Uppercase:", strings.ToUpper(text))
	fmt.Println("Lowercase:", strings.ToLower(text))
	fmt.Println("Word count:", len(strings.Fields(text)))
	fmt.Println("Contains 'secure':", strings.Contains(text, "secure"))
}`

	result3, err := executeCode(client, "go", stringCode)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	} else {
		fmt.Printf("Exit Code: %d\n", result3.ExitCode)
		fmt.Printf("Stdout: %s", result3.Stdout)
		if result3.Stderr != "" {
			fmt.Printf("Stderr: %s", result3.Stderr)
		}
	}
	fmt.Println()

	// Test 4: Error handling
	fmt.Println("=== Test 4: Error Handling ===")
	errorCode := `package main

import "fmt"

func main() {
	// This will cause a runtime panic
	var slice []int
	fmt.Println("Attempting to access slice[0]...")
	fmt.Println(slice[0])
}`

	result4, err := executeCode(client, "go", errorCode)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	} else {
		fmt.Printf("Exit Code: %d\n", result4.ExitCode)
		fmt.Printf("Stdout: %s", result4.Stdout)
		if result4.Stderr != "" {
			fmt.Printf("Stderr: %s", result4.Stderr)
		}
	}
	fmt.Println()

	// Test 5: Concurrent operations
	fmt.Println("=== Test 5: Concurrent Operations ===")
	concurrentCode := `package main

import (
	"fmt"
	"sync"
	"time"
)

func worker(id int, wg *sync.WaitGroup) {
	defer wg.Done()
	fmt.Printf("Worker %d starting\n", id)
	time.Sleep(time.Millisecond * 100)
	fmt.Printf("Worker %d done\n", id)
}

func main() {
	var wg sync.WaitGroup
	
	for i := 1; i <= 3; i++ {
		wg.Add(1)
		go worker(i, &wg)
	}
	
	wg.Wait()
	fmt.Println("All workers completed!")
}`

	result5, err := executeCode(client, "go", concurrentCode)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	} else {
		fmt.Printf("Exit Code: %d\n", result5.ExitCode)
		fmt.Printf("Stdout: %s", result5.Stdout)
		if result5.Stderr != "" {
			fmt.Printf("Stderr: %s", result5.Stderr)
		}
	}
	fmt.Println()

	fmt.Println("✅ All tests completed!")
}
