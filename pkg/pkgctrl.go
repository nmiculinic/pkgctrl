package pkg

import "context"

type Config struct {
	Providers []Provider
}

type Provider struct {
	Name        string    `json:"name"`
	IgnoreList  []Package `json:"ignore"`
	PresentList []Package `json:"present"`
}

type Package struct {
	Name        string
	Description string
}

type Controller struct {
	cfg *Config
}

func New(config *Config) *Controller {
	return &Controller{
		cfg: config,
	}
}

func (c *Controller) Run(ctx context.Context) error {
	return nil
}
