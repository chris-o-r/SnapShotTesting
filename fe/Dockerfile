# Stage 1: Build the React frontend
FROM node:latest AS builder

# Set the working directory
WORKDIR /app


# Copy the package.json and package-lock.json files
COPY package.json package-lock.json ./

# Remove the .env
RUN rm -f .env

# Install dependencies
RUN npm install

# Copy the source code
COPY . .

# Build the application
RUN npm run build

# Stage 2: Create a lightweight image to serve the built files
FROM nginx:alpine

# Copy the built files from the builder stage
COPY --from=builder /app/dist /usr/share/nginx/html

# Expose the HTTP port
EXPOSE 80

# Command to run Nginx
CMD ["nginx", "-g", "daemon off;"]
