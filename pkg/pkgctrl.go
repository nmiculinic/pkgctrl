package pkg

import (
	"context"
	"time"
)

type Config struct {
	Providers []Provider
}

type Provider struct {
	Name        string    `json:"name"`
	IgnoreList  []Package `json:"ignore"`
	PresentList []Package `json:"present"`
}

type Package struct {
	Name                 string
	Description          string
	Version              string
	Architecture         string
	URL                  string
	Licenses             []string
	Groups               []string
	Provides             []string
	Dependencies         []string
	OptionalDependencies []string
	RequiredBy           []string
	OptionalFor          []string
	ConflictsWith        []string
	Replaces             []string
	InstalledSizeBytes   int64
	Packager             string
	BuildDate            time.Time
	InstallDate          time.Time
	InstallReason        string
	ValidatedBy          string
	InstallScript        bool
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
