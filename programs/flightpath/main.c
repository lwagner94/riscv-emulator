#include "../common/common.h"

#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>
#include <ctype.h>
#include <stdint.h>

int errno = 0;
#include <errno.h>


#define SIZE_X (800)
#define SIZE_Y (600)
#define BYTE_PER_PIXEL (3)

#define INITIAL_BUFFER_SIZE         (20)

// We accept spaces AND tabs as delimiters
#define STRING_DELIMITERS           (" \t")

#define SUCCESS                     (0)
#define ERROR_INVALID_ARGUMENTS     (1)
#define ERROR_OUT_OF_MEMORY         (2)
#define ERROR_FILE_WRITE            (3)
#define ERROR_SPEED_NEGATIVE        (4)
#define ERROR_CONVERSION            (5)


#define DEFAULT_GRAVITATION         (9.798)
#define DEFAULT_HEIGHT              (320)
#define DEFAULT_WIDTH               (320)
#define DEFAULT_PPS                 (5)
#define DEFAULT_WIND_ANGLE          (0)
#define DEFAULT_WIND_FORCE          (0)


#define SEA_COLOUR_A          (0x3354B5)
#define SEA_COLOUR_B          (0x000050)
#define SUN_COLOUR            (0xFFFF99)
#define SUN_CORONA_COLOUR     (0xFFFFCC)
#define SKY_COLOUR_A          (0x3399FF)
#define SKY_COLOUR_B          (0xADD6FF)
#define START_COLOUR          (0xCDEFFF)
#define PATH_COLOUR           (0xFFFFFF)


// Get the colour components for predefined colours
#define RED_COMPONENT(colour)   ((colour & 0xFF0000) >> 16)
#define GREEN_COMPONENT(colour) ((colour & 0x00FF00) >> 8)
#define BLUE_COMPONENT(colour)   (colour & 0x0000FF)
#define INTITIALIZE_COLOUR(colour) {RED_COMPONENT(colour), \
          GREEN_COMPONENT(colour), BLUE_COMPONENT(colour)}


int end = 1048576 * 16;



int
_write (int   file,
        char *ptr,
        int   len)
{
//    errno = ENOSYS;
    debug(ptr);

    return len;
}


int
_close (int fildes)
{
    errno = ENOSYS;
    return -1;
}

int
_fstat (int          fildes,
        struct stat *st)
{
    errno = ENOSYS;
    return -1;
}

int
_isatty (int file)
{
    errno = ENOSYS;
    return 0;
}

int
_lseek (int   file,
        int   ptr,
        int   dir)
{
    errno = ENOSYS;
    return -1;
}

int
_read (int   file,
       char *ptr,
       int   len)
{
    errno = ENOSYS;
    return -1;
}


typedef struct _CommandLineParameters_
{
    float angle_;
    float speed_;
    char* output_filename_;
    char* config_filename_;

} CommandLineParameters;

typedef struct _Configuration_
{
    unsigned int image_width_;
    unsigned int image_height_;
    unsigned int points_per_second_;
    float gravitation_;
    float wind_angle_;
    float wind_force_;
} Configuration;

typedef struct _Vector_
{
    float x_coordinate_;
    float y_coordinate_;
} Vector;

typedef struct _CalculatedPositions_
{
    Vector* points_;
    unsigned int count_;
} CalculatedPositions;

typedef struct _Pixel_
{
    unsigned char red_;
    unsigned char green_;
    unsigned char blue_;
} Pixel;

typedef struct _ImageMatrix_
{
    Pixel** pixels_;
    unsigned int width_;
    unsigned int height_;
} ImageMatrix;

typedef struct _FileHeader_
{
    unsigned char bf_type_1_;
    unsigned char bf_type_2_;
    unsigned int bf_size_;
    unsigned int bf_reserved_;
    unsigned int bf_off_bits_;
} __attribute__((__packed__)) FileHeader;

typedef struct _BitmapInfoHeader_
{
    unsigned int bi_size_;
    unsigned int bi_width_;
    unsigned int bi_height_;
    unsigned short bi_planes_;
    unsigned short bi_bit_count_;
    unsigned int bi_compression_;
    unsigned int bi_size_image_;
    unsigned int bi_x_pels_per_meter_;
    unsigned int bi_y_pels_per_meter_;
    unsigned int bi_clr_used_;
    unsigned int bi_clr_important_;
} __attribute__((__packed__)) BitmapInfoHeader;

typedef struct _BitmapHeader_
{
    FileHeader file_header_;
    BitmapInfoHeader info_header_;
} __attribute__((__packed__)) BitmapHeader;

typedef struct _TokenizedString_
{
    char** strings_;
    unsigned int count_;
} TokenizedString;

// Public interface definitions

int parseCommandLineParameters(CommandLineParameters* parameters,
                               int argument_count,
                               char** arguments);

int parseConfiguration(Configuration* config,
                       CommandLineParameters* parameters);

int calculateFlightPath(CalculatedPositions* positions,
                        Configuration* config,
                        CommandLineParameters* parameters);

int prepareImageMatrix(ImageMatrix* matrix, Configuration* config);

void drawFlightPath(ImageMatrix* matrix, CalculatedPositions* positions);

int writeBitmap(ImageMatrix* matrix, Configuration* config,
                CommandLineParameters* parameters);


uint32_t end_of_heap = 1024 * 1024 * 100;
uint32_t start_of_heap = 1024 * 1024;


void *_sbrk(intptr_t increment) {
    if (start_of_heap + increment >= end_of_heap)
        return (void*) -1;
    uint32_t temp = start_of_heap;
    start_of_heap += increment;
    return (void*)temp;
}



//-----------------------------------------------------------------------------
///
/// Set all standard parameters.
///
/// @param config A pointer to an instace of the struct Configuration.
//
void setStandardValues(Configuration* config)
{
    config->gravitation_ = DEFAULT_GRAVITATION;
    config->image_height_ = DEFAULT_HEIGHT;
    config->image_width_= DEFAULT_WIDTH;
    config->points_per_second_ = DEFAULT_PPS;
    config->wind_angle_ = DEFAULT_WIND_ANGLE;
    config->wind_force_ = DEFAULT_WIND_FORCE;
}




//-----------------------------------------------------------------------------
///
/// Prepares the bitmap header and the fill data if necessary.
/// Writes the bitmap file.
///
/// @param matrix The 2 dimensional matrix containing the pixel data.
/// @param config The parsed configuration data.
/// @param parameters The parsed command line data.
///
/// @return int ERROR_OUT_OF_MEMORY if Out-of-memory error occurs,
///         SUCCESS if successful.
//


int writeBitmap(ImageMatrix* matrix,
                Configuration* config,
                CommandLineParameters* parameters)
{

        for (int line = 0; line < config->image_height_; line++)
        {
            for (int col = 0; col < config->image_width_; col++) {
                Pixel p = matrix->pixels_[line][col];
                uint32_t temp = 0;

                temp = p.red_ << 24;
                temp |= p.green_ << 16;
                temp |= p.blue_ << 8;

                draw_pixel(col, line, temp);
            }
        }
}

//-----------------------------------------------------------------------------
///
/// Interpolates between two numbers.
///
/// @param percent Percent value for interpolating.
/// @param number_1 Start number.
/// @param number_2 End number.
///
/// @return int The interpolated number.
//
int interpolate(float percent, int number_1, int number_2)
{
    if (percent > 1)
    {
        percent = 1;
    }
    if (percent < 0)
    {
        percent = 0;
    }
    return (1 - percent) * number_1 + percent * number_2;
}

//-----------------------------------------------------------------------------
///
/// Draws the background of the image.
///
/// @param matrix The 2 dimensional matrix containing the pixel data.
///
//
void drawBackground(ImageMatrix* matrix)
{
    int line = 0;
    int column = 0;

    for (line = 0; line < matrix->height_; line++)
    {
        for (column = 0; column < matrix->width_; column++)
        {
            Pixel pixel;
            float wave_offset = (matrix->height_ / 3.4) * 3;
            float wave_multiplier = (matrix->height_ / 100.0) * 2;
            float wave_sin = sin(column / (matrix->height_ / 12.0)) +
                             0.02 * sin(column / (matrix->height_ / 400.0));
            float wave_border = (wave_offset + wave_multiplier * wave_sin);
            // Checks if the pixel is part of the water or the sky.
            if (line > wave_border)
            {
                float border_diff = line - wave_border;
                float percent_wave = border_diff / (matrix->height_ - wave_border);
                // Simple Antialiasing.
                if(border_diff <= 3)
                {
                    pixel.red_ = interpolate((border_diff / 2),
                                             interpolate(line / (float) matrix->height_,
                                                         RED_COMPONENT(SKY_COLOUR_A), RED_COMPONENT(SKY_COLOUR_B)),
                                             RED_COMPONENT(SEA_COLOUR_A));

                    pixel.green_ = interpolate((border_diff / 2),
                                               interpolate(line / (float) matrix->height_,
                                                           GREEN_COMPONENT(SKY_COLOUR_A), GREEN_COMPONENT(SKY_COLOUR_B)),
                                               GREEN_COMPONENT(SEA_COLOUR_A));

                    pixel.blue_ = interpolate((border_diff / 2),
                                              interpolate(line / (float) matrix->height_,
                                                          BLUE_COMPONENT(SKY_COLOUR_A), BLUE_COMPONENT(SKY_COLOUR_B)),
                                              BLUE_COMPONENT(SEA_COLOUR_A));
                }
                else
                {
                    pixel.red_ = interpolate(percent_wave,
                                             RED_COMPONENT(SEA_COLOUR_A), RED_COMPONENT(SEA_COLOUR_B));
                    pixel.green_ = interpolate(percent_wave,
                                               GREEN_COMPONENT(SEA_COLOUR_A), GREEN_COMPONENT(SEA_COLOUR_B));
                    pixel.blue_ = interpolate(percent_wave,
                                              BLUE_COMPONENT(SEA_COLOUR_A), BLUE_COMPONENT(SEA_COLOUR_B));
                }
            }
            else
            {
                pixel.red_ = interpolate(line / (float)  matrix->height_,
                                         RED_COMPONENT(SKY_COLOUR_A), RED_COMPONENT(SKY_COLOUR_B));
                pixel.green_ = interpolate(line / (float)  matrix->height_,
                                           GREEN_COMPONENT(SKY_COLOUR_A), GREEN_COMPONENT(SKY_COLOUR_B));
                pixel.blue_ = interpolate(line / (float)  matrix->height_,
                                          BLUE_COMPONENT(SKY_COLOUR_A), BLUE_COMPONENT(SKY_COLOUR_B));
            }
            matrix->pixels_[line][column] = pixel;
        }
    }
}

//-----------------------------------------------------------------------------
///
/// Draws a blurred circle.
///
/// @param matrix The 2 dimensional matrix containing the pixel data.
/// @param offset_x X-coordinate of the center.
/// @param offset_y Y-coordinate of the center.
/// @param radius Radius of the circle.
/// @param new_color Pixel containing the wanted color.
///
//
void drawBlurredCircle(ImageMatrix* matrix, int offset_x, int offset_y,
                       int radius, Pixel new_color)
{
    int line = 0;
    int column = 0;

    for (line = offset_y - radius; line <= offset_y + radius; line++)
    {
        for (column = offset_x - radius; column <= offset_x + radius; column++)
        {
            // Calculate the absolute value of the vector
            // and compare it with the given radius.
            float abs_radius = sqrt(pow(column - offset_x, 2) +
                                    pow(line - offset_y, 2));
            // Also check if point is within the coordinate system.
            if (abs_radius <= radius && column >= 0 && column < matrix->width_ &&
                line >= 0 && line < matrix->height_)
            {
                float percentage = abs_radius / radius;
                Pixel pixel = matrix->pixels_[line][column];

                pixel.red_ = interpolate(percentage, new_color.red_, pixel.red_);
                pixel.green_ = interpolate(percentage, new_color.green_, pixel.green_);
                pixel.blue_ = interpolate(percentage, new_color.blue_, pixel.blue_);

                matrix->pixels_[line][column] = pixel;
            }
        }
    }
}

//-----------------------------------------------------------------------------
///
/// Draws the foreground of the bitmap.
///
/// @param matrix The 2 dimensional matrix containing the pixel data.
/// @param config The parsed configuration data.
///
//
void drawForeground(ImageMatrix* matrix)
{
    // Draw the center of the bitmap.

    int offset_x = matrix->width_ / 2;
    int offset_y = matrix->height_ / 2;
    int radius = matrix->width_ / 50;

    Pixel start_colour = INTITIALIZE_COLOUR(START_COLOUR);

    // Draw it twice so we get sharper contoures
    drawBlurredCircle(matrix, offset_x, offset_y, radius, start_colour);
    drawBlurredCircle(matrix, offset_x, offset_y, radius, start_colour);

    // Draw the sun.
    Pixel sun_colour = INTITIALIZE_COLOUR(SUN_COLOUR);
    Pixel corona_colour = INTITIALIZE_COLOUR(SUN_CORONA_COLOUR);
    offset_x = matrix->width_ / 10;
    offset_y = matrix->height_ / 50;
    radius = matrix->width_ / 12;

    drawBlurredCircle(matrix, offset_x, offset_y, ceil(radius * 2.5),
                      corona_colour);
    // Draw it twice so we get sharper contoures
    drawBlurredCircle(matrix, offset_x, offset_y, radius, sun_colour);
    drawBlurredCircle(matrix, offset_x, offset_y, radius, sun_colour);


}

//-----------------------------------------------------------------------------
///
/// Prepares the 2 dimensional image matrix.
///
/// @param matrix The 2 dimensional matrix containing the pixel data.
/// @param config The parsed configuration data.
///
/// @return ERROR_OUT_OF_MEMORY if Out-of-memory error occurs,
///         SUCCESS if successful.
//
int prepareImageMatrix(ImageMatrix* matrix, Configuration* config)
{
    matrix->height_ = config->image_height_;
    matrix->width_ = config->image_width_;

    // Allocate memory for the lines.
    matrix->pixels_ = malloc(matrix->height_ * sizeof(Pixel*));
    if (matrix->pixels_ == NULL)
    {
        return ERROR_OUT_OF_MEMORY;
    }

    // Allocate memory for the columns.
    int line = 0;
    for (line = 0; line < matrix->height_; line++)
    {
        matrix->pixels_[line] = malloc(matrix->width_ * sizeof(Pixel));
        if (matrix->pixels_[line] == NULL)
        {
            return ERROR_OUT_OF_MEMORY;
        }
    }

    drawBackground(matrix);
    drawForeground(matrix);

    return SUCCESS;
}

//-----------------------------------------------------------------------------
///
/// Interpolate between calculated positions and draw the points into the
/// ImageMatrix.
///
/// @param matrix A pointer to an ImageMatrix.
/// @param positions A pointer to a struct of the type CalcualtedPositions.
//
void drawFlightPath(ImageMatrix* matrix, CalculatedPositions* positions)
{
    int count = 0;
    int origin_x = matrix->width_ / 2;
    int origin_y = matrix->height_ / 2;

    float last_x = positions->points_[0].x_coordinate_;
    float last_y = positions->points_[0].y_coordinate_;

    for (count = 1; count < positions->count_; count++)
    {
        float current_x = positions->points_[count].x_coordinate_;
        float current_y = positions->points_[count].y_coordinate_;

        float delta_x = current_x - last_x;
        float delta_y = current_y - last_y;

        float distance = sqrt((delta_x * delta_x) + (delta_y * delta_y));
        int interpolation_steps = ceil(distance * 0.03);


        int interpolation_counter;
        for (interpolation_counter = 0;
             interpolation_counter < interpolation_steps;
             interpolation_counter++)
        {
            float interpolation_percentage = (1.0 / interpolation_steps) *
                                             interpolation_counter;

            int position_x = interpolate(interpolation_percentage,
                                         last_x, current_x) / 10;
            int position_y = -interpolate(interpolation_percentage,
                                          last_y, current_y) / 10;

            Pixel colour = INTITIALIZE_COLOUR(PATH_COLOUR);

            drawBlurredCircle(matrix, origin_x + position_x,
                              origin_y + position_y, 4, colour);
        }

        last_x = current_x;
        last_y = current_y;
    }
}

//-----------------------------------------------------------------------------
///
/// Function to perform a Multiplication between a Vector and a Scalar.
///
/// @param result_vector Pointer to a Vector for returning result.
/// @param vector Vector as factor for Multiplication.
/// @param scalar Scalar as factor for Multiplication.
//
void vectorScalarMultiplication(Vector* result_vector, Vector* vector,
                                float scalar)
{
    result_vector->x_coordinate_ = vector->x_coordinate_ * scalar;
    result_vector->y_coordinate_ = vector->y_coordinate_ * scalar;
}

//-----------------------------------------------------------------------------
///
/// Function to perform an Addition between two Vectors.
///
/// @param result_vector Pointer to a Vector for returning result.
/// @param vector_1 First summand for Vector Addition.
/// @param vector_2 Second summand for Vector Addition.
//
void vectorVectorAddition(Vector* result_vector, Vector* vector_1,
                          Vector* vector_2)
{
    result_vector->x_coordinate_ =
            vector_1->x_coordinate_ + vector_2->x_coordinate_;
    result_vector->y_coordinate_ =
            vector_1->y_coordinate_ + vector_2->y_coordinate_;
}

//-----------------------------------------------------------------------------
///
/// This function converts a given value in degrees into radians.
///
/// @param degrees The value in degrees.
///
/// @return float The converted value in radians.
//
float degreesToRadians(float degrees)
{
    return (degrees * M_PI) / 180;
}

//-----------------------------------------------------------------------------
///
/// Initializes a Vector from a given angle and length, using simple
/// trigonometry.
///
/// @param result_vector Pointer to a Vector for returning result.
/// @param length The desired length of the new Vector.
/// @param angle The desired angle of the new Vector.
//
void initializeVector(Vector* result_vector, float length, float angle)
{
    angle = degreesToRadians(angle);

    result_vector->x_coordinate_ = cos(angle) * length;
    result_vector->y_coordinate_ = sin(angle) * length;
}

//-----------------------------------------------------------------------------
///
/// Converts the x and y coordinates from a vector, given in meters, into
/// pixels.
///
/// @param result_vector Pointer to a Vector for returning converted Vector.
/// @param vector Vector that should be converted.
//
void metersToPixels(Vector* result_vector, Vector* vector)
{
    result_vector->x_coordinate_ = vector->x_coordinate_ / 10;
    result_vector->y_coordinate_ = vector->y_coordinate_ / 10;
}

//-----------------------------------------------------------------------------
///
/// Perform the actual calculation of the next vector.
///
/// @param result_vector Pointer to the vector that will be calculated.
/// @param speed Vector that contains information about the speed.
/// @param gravitation Vector that contains information about the gravitation.
/// @param wind Vector that contains information about the wind.
/// @param time The current time that is used for the calculation.
//
void calculateNextVector(Vector* result_vector, Vector* speed,
                         Vector* gravitation, Vector* wind, float time)
{
    // Auxiliary variables
    Vector speed_applied;
    Vector gravitation_applied;
    Vector wind_applied;

    result_vector->x_coordinate_ = 0;
    result_vector->y_coordinate_ = 0;
    // Actual Calculation
    vectorScalarMultiplication(&speed_applied, speed, time);

    vectorScalarMultiplication(&gravitation_applied, gravitation,
                               time * time / 2);

    vectorScalarMultiplication(&wind_applied, wind, time * time / 2);

    vectorVectorAddition(result_vector, &speed_applied, &gravitation_applied);
    vectorVectorAddition(result_vector, result_vector, &wind_applied);
}

//-----------------------------------------------------------------------------
///
/// This function calculates the flightpath of the projectile using data from
/// the config file and command line parameters (speed, angle, wind and
/// gravitation). The PPS Parameter (Points per second) determines the time
/// between two calculated points.
///
/// @param positions Pointer for returning calculated positions.
/// @param config Contains all the information from the config file.
/// @param parameters Contains all the information from the command line.
///
/// @return int ERROR_OUT_OF_MEMORY if Out-of-memory error occurs,
///             SUCCESS if successful.
//
int calculateFlightPath(CalculatedPositions* positions, Configuration* config,
                        CommandLineParameters* parameters)
{
    Vector speed;
    initializeVector(&speed, parameters->speed_, parameters->angle_);

    Vector gravitation;
    initializeVector(&gravitation, config->gravitation_, 270);

    Vector wind;
    initializeVector(&wind, config->wind_force_, config->wind_angle_);

    float time_step = 1 / (float)config->points_per_second_;
    float current_time;

    Vector position_meters;
    Vector* pointer_backup;

    positions->count_ = 0;

    // Allocate initial memory
    unsigned int capacity = INITIAL_BUFFER_SIZE;
    positions->points_ = malloc(capacity  * sizeof(Vector));

    if (positions->points_ == NULL)
    {
        return ERROR_OUT_OF_MEMORY;
    }

    do
    {
        // Get more memory if needed
        if (positions->count_ + 1 >= capacity)
        {
            pointer_backup = positions->points_;
            capacity *= 2;
            positions->points_ = realloc(positions->points_,
                                         capacity * sizeof(Vector));

            if (positions->points_ == NULL)
            {
                free(pointer_backup);
                return ERROR_OUT_OF_MEMORY;
            }
        }

        current_time = (positions->count_ + 1) * time_step;
        calculateNextVector(&positions->points_[positions->count_], &speed,
                            &gravitation, &wind, current_time);

        metersToPixels(&position_meters,
                       &positions->points_[positions->count_]);
        // Increment Point Counter
        positions->count_++;

        // Abort Calculation if point exits image
    }
    while (fabs(position_meters.x_coordinate_) < (config->image_width_ / 2) &&
           fabs(position_meters.y_coordinate_) < (config->image_height_ / 2));

    // Free unused memory
    pointer_backup = positions->points_;
    positions->points_ =
            realloc(positions->points_, positions->count_ * sizeof(Vector));

    if (positions->points_ == NULL)
    {
        free(pointer_backup);
        return ERROR_OUT_OF_MEMORY;
    }
    return SUCCESS;
}

//-----------------------------------------------------------------------------
///
/// Free all 'global' ressources
///
/// @param matrix The image matrix containing pixel data.
/// @param positions The struct containing all calculated positions.
//
void freeResources(ImageMatrix* matrix, CalculatedPositions* positions)
{
    int counter = 0;

    // Protect from segfaults during freeing.
    if (matrix->pixels_ != NULL)
    {
        for (counter = 0; counter < matrix->height_; counter++)
        {
            free(matrix->pixels_[counter]);
            matrix->pixels_[counter] = NULL;
        }
    }

    free(matrix->pixels_);
    matrix->pixels_ = NULL;

    free(positions->points_);
    positions->points_ = NULL;
}

//-----------------------------------------------------------------------------
///
/// Display an appropriate error message for each error.
///
/// @param error_code The error code.
//
void displayErrorMessage(int error_code)
{
//    switch (error_code)
//    {
//        case ERROR_INVALID_ARGUMENTS:
//            printf("usage: ./assa [float:angle] [float:speed] [output_filename] "
//                   "{optional:config_filename}\n");
//            break;
//        case ERROR_OUT_OF_MEMORY:
//            printf("error: out of memory\n");
//            break;
//        case ERROR_FILE_WRITE:
//            printf("error: couldn't write file\n");
//            break;
//        case ERROR_SPEED_NEGATIVE:
//            printf("error: speed must be > 0\n");
//            break;
//
//        default:
//            break;
//    }
}



//-----------------------------------------------------------------------------
///
/// Main function.
/// Parses the command line arguments and then optionally a config file.
/// After that, we calculate all points, prepare the ImageMatrix,
/// interpolate and draw all points and finally we write the bitmanp.
///
/// @argc Absoulute count of all command line arguments.
/// @argv array String array of all command line arguments
///
/// @return SUCCESS (0) if no error occurred, or some error code.
//
void notmain(void)
{
    CommandLineParameters parameters;
    Configuration config;
    CalculatedPositions positions;
    ImageMatrix matrix;

    positions.points_ = NULL;
    matrix.pixels_ = NULL;

    int error = SUCCESS;


    parameters.angle_ = 75;
    parameters.speed_ = 150;
    config.gravitation_ = DEFAULT_GRAVITATION;
    config.image_height_ = SIZE_Y;
    config.image_width_ = SIZE_X;
    config.points_per_second_ = DEFAULT_PPS;
    config.wind_angle_ = DEFAULT_WIND_ANGLE;
    config.wind_force_ = DEFAULT_WIND_FORCE;


    if (!error)
    {
        puts("Flight path\n");
        error = calculateFlightPath(&positions, &config, &parameters);
    }
    if (!error)
    {
        puts("prepare\n");
        error = prepareImageMatrix(&matrix, &config);
    }
    if (!error)
    {
        // drawFligthPath does not raise any errors.
        puts("draw\n");
        drawFlightPath(&matrix, &positions);
        error = writeBitmap(&matrix, &config, &parameters);
    }

    if (error)
    {
        displayErrorMessage(error);
    }

    freeResources(&matrix, &positions);

    for (;;) ;

    return error;
}